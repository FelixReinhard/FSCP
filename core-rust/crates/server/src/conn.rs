use shared::{config, errors::Error};
use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{Server, util};

pub fn serve_server(pfx_path: &str, address: &str, server: Server) -> Result<(), Error> {
    // first pfx file.
    let identity = util::load_identity(pfx_path)?;
    // create listener and acceptor
    let (listener, acceptor) = util::create_listener(address, identity)?;


    let current_client_number = Arc::new(Mutex::new(0));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let mut current_client_number = current_client_number.lock().unwrap();
                if *current_client_number < config::MAX_CLIENTS {
                    *current_client_number += 1;
                    // Spawn new thread to handle stream.
                    thread::spawn(move || {
                        let stream = acceptor.accept(stream).unwrap();
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
