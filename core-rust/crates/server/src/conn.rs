use native_tls::TlsStream;
use shared::{config, errors::Error};
use std::{
    io::Read,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use crate::{Server, helper::ServerHelper, util};

pub fn serve_server(pfx_path: &str, address: &str, server: ServerHelper) -> Result<(), Error> {
    // first pfx file.
    let identity = util::load_identity(pfx_path)?;
    // create listener and acceptor
    let (listener, acceptor) = util::create_listener(address, identity)?;

    let server = Arc::new(Mutex::new(server));

    let current_client_number = Arc::new(Mutex::new(0));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let mut current_client_number = current_client_number.lock().unwrap();
                if *current_client_number < config::MAX_CLIENTS {
                    *current_client_number += 1;
                    let server = server.clone();
                    // Spawn new thread to handle stream.
                    thread::spawn(move || {
                        let stream = acceptor.accept(stream).unwrap();
                        handle(stream, server);
                    });
                }
            }
            Err(_) => {
                // TODO log error.
            }
        }
    }

    Ok(())
}

fn handle(mut stream: TlsStream<TcpStream>, server: Arc<Mutex<ServerHelper>>) -> Result<(), Error> {
    // First init and get the sending and recieving
    let (to_server, from_server);
    {
        (to_server, from_server) = server.lock().unwrap().register()?;
    }

    loop {
        // first recieve the length.
        let mut len_buffer = [0u8; 4];
        stream
            .read_exact(&mut len_buffer)
            .expect("Should be able to read from stream.");
    }
}
