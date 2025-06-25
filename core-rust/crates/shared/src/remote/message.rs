// This file contains the code for Messages that are send between client and server.

use uuid::Uuid;

use crate::datatypes::Data;

pub enum Message {
    // ClientHello(u8, Option<RsaPublicKey>), // version, optional public key. Functions as the
    // // clients certificate.
    // ServerHello([u8; 32], [u8; 256]), // DH message, nonce encrypted with public key from previous message. To
    // // be decrypted by client and encrypted with generated shared key and sent back.
    // // This ensures the client is the owner of certificate and has the shared aes key.
    // ClientAuth([u8; 32], [u8; 256]), // DH message, nonce decrypted and encrypted with shared
    // // session key.
    ServerChange(TreeChange),
    ServerLog(String), // Is send to inform client of succesffull button press or any erros.
    ClientHash(u64),   // Sends the hash of the tree. If Server sees difference check saved hashes
    // and resend.
    ClientTrigger(Uuid), // Tries to trigger a Button node.
}
