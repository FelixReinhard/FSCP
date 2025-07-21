use std::{fs::File, net::TcpListener, sync::Arc};

use native_tls::{Identity, TlsAcceptor};
use shared::errors::Error;
use std::io::Read;

/// Load the Identiy struct from a pfx file. (Pfx is generated from key and certificate)
pub fn load_identity(pfx_path: &str) -> Result<Identity, Error> {
    let mut file = if let Ok(file) = File::open(pfx_path) {
        file
    } else {
        return Err(Error::SimpleErrorStr(format!(
            "Couldnt open pfx file {pfx_path}"
        )));
    };
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = if let Ok(Identity) = Identity::from_pkcs12(&identity, "") {
        identity
    } else {
        return Err(Error::SimpleErrorStr(format!(
            r#"Couldnt open pfx file {pfx_path}. Maybe password != """#
        )));
    };

    let identity = if let Ok(identity) = Identity::from_pkcs12(&identity, "hunter2") {
        identity
    } else {
        return Err(Error::SimpleErrorStr(format!(
            "Couldnt create identity from bytes {:?}",
            identity
        )));
    };

    Ok(identity)
}

pub fn create_listener(
    addr: &str,
    identity: Identity,
) -> Result<(TcpListener, Arc<TlsAcceptor>), Error> {
    let listener = if let Ok(l) = TcpListener::bind(addr) {
        l
    } else {
        return Err(Error::SimpleErrorStr(format!(
            "Couldnt bind with address: {addr}"
        )));
    };

    let acceptor = if let Ok(a) = TlsAcceptor::new(identity) {
        a
    } else {
        return Err(Error::SimpleErrorStr(format!(
            "Couldnt create acceptor with address: {addr}"
        )));
    };
    let acceptor = Arc::new(acceptor);
    Ok((listener, acceptor))
}
