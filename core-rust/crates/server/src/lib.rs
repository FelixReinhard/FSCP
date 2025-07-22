mod conn;
mod handler;
mod helper;
mod internal_message;
mod util;

use std::thread;

use crossbeam::channel::{Receiver, RecvError, SendError, Sender, select_biased};
use shared::{
    datatypes::{nodes::Node, treebuilder::TreeChange},
    errors::Error,
};

use crate::{handler::ServerHandler, helper::ServerHelper, internal_message::InternalMessage};

// Server gives out channel pairs for each client connection. These channels connect to.
// Server gives out channel pairs for each client connection. These channels connect to.
pub struct Server {
    helper: ServerHelper,
    address: &'static str,
}

impl Server {
    pub fn new() -> Self {
        let (to_handler_s, to_handler_r) = crossbeam::channel::unbounded::<InternalMessage>();
        let (to_server_s, to_server_r) = crossbeam::channel::unbounded::<InternalMessage>();
        let (from_clients_to_handler_s, from_clients_to_handler_r) =
            crossbeam::channel::unbounded::<InternalMessage>();

        let server_thread = thread::spawn(move || {
            match ServerHandler::new(to_server_s, to_handler_r, from_clients_to_handler_r).run() {
                Err(err) => println!("{err:?}"),
                _ => {}
            };
        });

        Self {
            helper: ServerHelper::new(to_handler_s, to_server_r, from_clients_to_handler_s),
            address: "localhost:8001",
        }
    }

    /// Serve the configured server and get a [RunningServer] struct. This has most of the
    /// functionality of the not running [Server], but acts more as another client with higher
    /// priotity.j
    pub fn serve(mut self) -> Result<RunningServer, Error> {
        let (s, r) = self.helper.register()?;
        thread::spawn(move || {
            match conn::serve_server("test.pfx", self.address, self.helper) {
                Err(err) => {
                    println!("{err:?}")
                }
                _ => {}
            };
        });
        Ok(RunningServer::new(s, r))
    }

    /// Add a child to the root node of the tree.
    pub fn add_child(&mut self, node: Node) {}
}

pub struct RunningServer {
    to_handler_s: Sender<InternalMessage>,
    from_handler_r: Receiver<InternalMessage>,
}

// When the server starts the helper is moved therefore a new struct is needed that also implements
// the ServerInterface trait (todo at this point lol)
impl RunningServer {
    fn new(
        to_handler_s: Sender<InternalMessage>,
        from_handler_r: Receiver<InternalMessage>,
    ) -> Self {
        Self {
            to_handler_s,
            from_handler_r,
        }
    }
}
