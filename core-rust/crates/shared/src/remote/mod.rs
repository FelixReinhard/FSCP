pub mod message;

use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use rsa::pkcs8::der::Writer;

use crate::{config, errors::Error};

/// Contains code that makes remote calls possible.

/// How many tcp connections can connect and be maintained.
const POSSIBLE_CONCURRENT_CONNECTIONS: u32 = 32;

pub struct TcpManager {
    running_threads_amount: u32,
    can_accept_new_connection: Condvar,
    shut_down: bool, // this is set by the main loop if we should shut down all connections.
}

/// Opens tcp listener and handles the streams.
pub fn tcp_listen(manager: Arc<Mutex<TcpManager>>) -> Result<(), Error> {
    let listener = match TcpListener::bind(config::get_ip()) {
        Ok(l) => l,
        Err(_) => return Err(Error::SimpleError("Cannot create tcp listener.")),
    };

    // indefinatly accept ...
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                // First we check if there are currently open slots for clients.
                {
                    // TODO proper error handling
                    if manager.lock().unwrap().running_threads_amount
                        >= POSSIBLE_CONCURRENT_CONNECTIONS
                    {
                        // Thre is no slot for a new connection. So terminate TODO error handling
                        s.shutdown(std::net::Shutdown::Both).unwrap();
                    }
                }
                // Create a clone of manager and move to new thread.
                let new_manager = manager.clone();
                thread::spawn(move || {
                    handle_tcp_stream(new_manager, s);
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

/// Any incoming tcp streams are here. Note that they all run in a seperate thread.
/// The manager contains the amount of
fn handle_tcp_stream(manager: Arc<Mutex<TcpManager>>, mut stream: TcpStream) {
    // First check if we are allowed to spawn a new thread.
    {
        let mut lock = if let Ok(l) = manager.lock() {
            l
        } else {
            return;
        };
        // incremet it.
        lock.running_threads_amount += 1;
    }

    // Handle the allowed stream as there are still slots open.
    handle_allowed_stream(stream);

    // Done with the handling, so decrement count
    {
        let mut lock = if let Ok(l) = manager.lock() {
            l
        } else {
            // This case might be really bad TODO TODO TODO WARNING AHHHHHH HELP ME LA POLIZIA LA
            // SIGMA SIGMA MAN OUHHHHHHHHHHHHHHHHHHH IIII
            return;
        };
        lock.running_threads_amount -= 1;
    }
}
/// The stream is allowed to happen so handle it.
fn handle_allowed_stream(mut stream: TcpStream) -> Result<(), Error> {
    match stream.set_read_timeout(Some(Duration::from_secs(10))) {
        Ok(()) => {}
        Err(error) => {
            drop(stream);
            return Err(Error::SimpleErrorStr(format!(
                "Tcp: When setting the read timeout somethign went bad: {:?}\n Closing connection.",
                error
            )));
        }
    };
    // First we need to accept the ClientHello
    // sizeof(ClientHello) =
    //
    //
    stream = client_hello(stream)?;
    Ok(())
}

// Do the client hello by recieving the message and acting accordingly.
fn client_hello(mut stream: TcpStream) -> Result<TcpStream, Error> {
    todo!()
}
