use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{config, errors::Error};

/// Contains code that makes remote calls possible.

/// Opens tcp listener and handles the streams.
pub fn tcp_listen() -> Result<(), Error> {
    let listener = match TcpListener::bind(config::get_ip()) {
        Ok(l) => l,
        Err(_) => return Err(Error::SimpleError("Binding tcp has not worked. TODO")),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // TODO limit how many listeners can be opened
                thread::spawn(|| {
                    handle_tcp_stream(s);
                });
            }
            Err(_) => {
                drop(listener);
                return Err(Error::SimpleError("Tcp Stream error happened. TODO"));
            }
        }
    }
    drop(listener);
    Ok(())
}

pub fn handle_tcp_stream(mut stream: TcpStream) {}
