// This file contains the code for Messages that are send between client and server.

use rsa::{RsaPublicKey, traits::PublicKeyParts};
use uuid::Uuid;

use crate::{config, datatypes::treebuilder::TreeChange};

pub enum Message {
    ClientHello(u8, Option<RsaPublicKey>), // version, optional public key. Functions as the
    // clients certificate.
    ServerHello([u8; 32], [u8; 256]), // DH message, nonce encrypted with public key from previous message. To
    // be decrypted by client and encrypted with generated shared key and sent back.
    // This ensures the client is the owner of certificate and has the shared aes key.
    ClientAuth([u8; 32], [u8; 256]), // DH message, nonce decrypted and encrypted with shared
    // session key.
    ServerChange(TreeChange), // This message is sent to communicate changes in the tree.
    ServerLog(String),        // Is send to inform client of succesffull button press or any erros.
    ClientHash(u64), // Sends the hash of the tree. If Server sees difference check saved hashes
    // and resend.
    ClientTrigger(Uuid), // Tries to trigger a Button node.
}

mod client_hello {
    use std::io::{BufRead, BufReader, Cursor, Read};

    use rand::thread_rng;
    use rsa::traits::PublicKeyParts;
    use rsa::{BigUint, RsaPrivateKey, RsaPublicKey};

    use byteorder::ReadBytesExt;

    use crate::config;
    use crate::errors::Error;

    /// The version is saved in the first seven bits of version byte
    /// Last bit is used for a flag. CERTIFICATE_PRESENT = 1|0
    /// CERTIFICATE_PRESENT = 1 -> there is a certificate.
    /// CERTIFICATE_PRESENT = 0 -> there is not certificate meaning all 0 bits
    ///
    ///
    /// byte 0: version and if there is a certificate.
    /// byte 1: len of n
    /// byte 2: len e
    fn serialize(version: u8, certificate: RsaPublicKey) -> Vec<u8> {
        let mut buf = vec![0; 4];

        buf[0] = (version << 1) | 0b1;

        // A quick check that I didn't change this in the config,
        // would then create problems as we assume n and e are combind max 259
        assert!(config::RSA_KEY_SIZE == 2048);

        let mut n = certificate.n().to_bytes_le();
        let mut e = certificate.e().to_bytes_le();

        assert_eq!(*certificate.n(), BigUint::from_bytes_le(&n));

        assert!(n.len() <= 256);
        assert!(e.len() <= 3);

        // Save casting as we asserted that both lens fit into u8
        let size_bytes = n.len().to_le_bytes();
        buf[1] = size_bytes[0];
        buf[2] = size_bytes[1];
        buf[3] = e.len() as u8;

        buf.append(&mut n);
        buf.append(&mut e);

        buf
    }

    fn serialize_client_hello_without_cert(version: u8) -> [u8; 1] {
        [version << 1]
    }

    fn deserialize<R: Read>(
        reader: &mut BufReader<R>,
    ) -> Result<(u8, Option<RsaPublicKey>), Error> {
        // Helper function to use ? syntax
        fn reader_error<T>(
            res: Result<T, std::io::Error>,
            message: &'static str,
        ) -> Result<T, Error> {
            match res {
                Ok(v) => Ok(v),
                Err(_err) => Err(Error::SimpleError(message)),
            }
        }

        let version_and_has_cert = reader_error(
            reader.read_u8(),
            "Deserialize ClientHello: cannot parse u8 for version_and_has_cert.",
        )?;

        let version = (version_and_has_cert & 0b11111110) >> 1;

        if version != config::CURRENT_VERSION {
            return Err(Error::SimpleErrorStr(format!(
                "Deserialize ClientHello: Wrong version: current {}, got: {version}",
                config::CURRENT_VERSION
            )));
        }
        let has_cert = version_and_has_cert & 0b1;

        if has_cert > 0 {
            // size of n component of RsaPublicKey
            // Is a u16
            let n_size_1 = reader_error(
                reader.read_u8(),
                "Deserialize ClientHello: cannot parse size of n.",
            )?;

            let n_size_2 = reader_error(
                reader.read_u8(),
                "Deserialize ClientHello: cannot parse size of n.",
            )?;

            let n_size: u16 = u16::from_le_bytes([n_size_1, n_size_2]);

            let e_size = reader_error(
                reader.read_u8(),
                "Deserialize ClientHello: cannot parse size of e.",
            )?;

            let mut n_bytes = vec![0u8; n_size as usize];
            let mut e_bytes = vec![0u8; e_size as usize];

            reader_error(
                reader.read_exact(&mut n_bytes),
                "Deserialize ClientHello: Couldnt read bytes for n.",
            )?;
            reader_error(
                reader.read_exact(&mut e_bytes),
                "Deserialize ClientHello: Couldnt read bytes for n.",
            )?;

            let n = BigUint::from_bytes_le(&n_bytes);

            let e = BigUint::from_bytes_le(&e_bytes);

            let key = match RsaPublicKey::new(n.clone(), e.clone()) {
                Ok(key) => key,
                Err(err) => {
                    return Err(Error::SimpleErrorStr(format!(
                        "Deserialize ClientHello: Couldnt create RsaPublicKey from {}(n) an {}(e) with {:?}",
                        n, e, err
                    )));
                }
            };

            Ok((version, Some(key)))
        } else {
            Ok((version, None))
        }
    }

    #[test]
    fn test_simple() {
        let n = 55u32; // modulus
        let e = 3u32; // public exponent
        let d = 27u32; // private exponent

        // Construct BigUint from u32
        let n = BigUint::from(n);
        let e = BigUint::from(e);
        let d = BigUint::from(d);

        // Small primes p and q that multiply to n (not strictly needed to encrypt/decrypt here)
        let p = BigUint::from(5u32);
        let q = BigUint::from(11u32);

        // Construct private key
        let private_key = RsaPrivateKey::from_components(n.clone(), e.clone(), d, vec![p, q]);

        // Construct public key
        let public_key = RsaPublicKey::new(n, e).unwrap();

        let serialized = serialize(1, public_key.clone());
        assert_eq!(serialized[0], 0b11);

        let mut reader = BufReader::new(Cursor::new(serialized));
        let (version, public_key2) = deserialize(&mut reader).unwrap();

        assert_eq!(version, 1);
        assert_eq!(public_key, public_key2.unwrap());
    }
    #[test]
    fn test() {
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, config::RSA_KEY_SIZE).unwrap();

        // Construct public key
        let public_key = RsaPublicKey::from(&private_key);

        let serialized = serialize(1, public_key.clone());
        assert_eq!(serialized[0], 0b11);

        let mut reader = BufReader::new(Cursor::new(serialized));
        let (version, public_key2) = deserialize(&mut reader).unwrap();

        assert_eq!(version, 1);
        assert_eq!(public_key, public_key2.unwrap());
    }
}
