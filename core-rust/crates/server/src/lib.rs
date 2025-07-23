mod conn;
mod handler;
mod helper;
mod internal_message;
pub mod server_interface;
mod util;

use std::thread;

use crossbeam::channel::{Receiver, RecvError, SendError, Sender, select_biased};
use shared::{
    datatypes::{nodes::Node, treebuilder::TreeChange},
    errors::Error,
    security::permissions::Permissions,
};
use uuid::Uuid;

use crate::{
    handler::ServerHandler, helper::ServerHelper, internal_message::InternalMessage,
    server_interface::ServerInterface,
};

// Server gives out channel pairs for each client connection. These channels connect to.
// Server gives out channel pairs for each client connection. These channels connect to.
pub struct Server {
    address: String,
    root: Node,
}

impl Server {
    pub fn new() -> Self {
        Self {
            address: "localhost:8001".to_string(),
            root: Node::new().name("root").permissions(Permissions::Public),
        }
    }

    /// Serve the configured server and get a [RunningServer] struct. This has most of the
    /// functionality of the not running [Server], but acts more as another client with higher
    /// priotity.
    pub fn serve(self) -> Result<RunningServer, Error> {
        // first set up the handler.
        let (to_handler_s, to_handler_r) = crossbeam::channel::unbounded::<InternalMessage>();
        let (to_server_s, to_server_r) = crossbeam::channel::unbounded::<InternalMessage>();
        let (from_clients_to_handler_s, from_clients_to_handler_r) =
            crossbeam::channel::unbounded::<InternalMessage>();

        // start the handler thread
        let _server_thread = thread::spawn(move || {
            match ServerHandler::new(to_server_s, to_handler_r, from_clients_to_handler_r).run() {
                Err(err) => println!("{err:?}"),
                _ => {}
            };
        });

        // create the helper struct to contain the channel end and start points.
        let mut helper = ServerHelper::new(to_handler_s, to_server_r, from_clients_to_handler_s);

        let (s, r) = helper.register()?;
        thread::spawn(move || {
            match conn::serve_server("test.pfx", &self.address, helper) {
                Err(err) => {
                    println!("{err:?}")
                }
                _ => {}
            };
        });
        Ok(RunningServer::new(s, r))
    }
}

impl ServerInterface for Server {
    /// Add a child to the root node of the tree.
    fn add_child(&mut self, node: Node) -> Result<(), Error> {
        self.root.add_child(node);
        Ok(())
    }

    /// Add a child to the node with parent_id to the tree.
    fn add_child_to_node(&mut self, node: Node, parent_id: &Uuid) -> Result<(), Error> {
        if let Some(parent) = self.root.find_node_mut(parent_id) {
            parent.add_child(node);
            Ok(())
        } else {
            Err(Error::SimpleErrorStr(format!(
                "Cannot find node {parent_id}"
            )))
        }
    }

    fn get_node(&self, id: &Uuid) -> Result<&Node, Error> {
        if let Some(n) = self.root.find_node(id) {
            Ok(n)
        } else {
            Err(Error::SimpleErrorStr(format!(
                "[get_node]: Couldnt find node with {id}"
            )))
        }
    }

    fn get_node_mut(&mut self, id: &Uuid) -> Result<&mut Node, Error> {
        if let Some(n) = self.root.find_node_mut(id) {
            Ok(n)
        } else {
            Err(Error::SimpleErrorStr(format!(
                "[get_node]: Couldnt find node with {id}"
            )))
        }
    }
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
