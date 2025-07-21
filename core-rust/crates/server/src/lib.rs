mod conn;
mod internal_messages;
mod util;

use core::fmt;
use std::{
    collections::HashMap,
    fmt::format,
    sync::{Arc, RwLock},
    thread,
};

use crossbeam::channel::{Receiver, RecvError, SendError, Sender, select_biased};
use shared::{
    datatypes::{nodes::Node, treebuilder::TreeChange},
    errors::Error,
};

use crate::internal_messages::Message;

// Server gives out channel pairs for each client connection. These channels connect to.
pub struct Server {
    to_handler_s: Sender<Message>,
    from_handler_r: Receiver<Message>,
    to_handler_from_clients_s: Sender<Message>,
}

impl Server {
    pub fn new() -> Self {
        let (to_handler_s, to_handler_r) = crossbeam::channel::unbounded::<Message>();
        let (to_server_s, to_server_r) = crossbeam::channel::unbounded::<Message>();
        let (from_clients_to_handler_s, from_clients_to_handler_r) =
            crossbeam::channel::unbounded::<Message>();

        let server_thread = thread::spawn(move || {
            ServerHandler::new(to_server_s, to_handler_r, from_clients_to_handler_r).run();
        });

        Self {
            to_handler_s,
            from_handler_r: to_server_r,
            to_handler_from_clients_s: from_clients_to_handler_s,
        }
    }

    /// Add a child to the root node of the tree.
    pub fn add_child(&mut self, node: Node) {}
}

// Handles the managment of the tree.
struct ServerHandler {
    root: Node,
    /// Map that connects an client id to a sending channel
    clients: HashMap<u64, Sender<Message>>,
    to_server_s: Sender<Message>,
    from_server_r: Receiver<Message>,
    from_clients_r: Receiver<Message>,
}

impl ServerHandler {
    fn new(
        to_server_s: Sender<Message>,
        from_server_r: Receiver<Message>,
        from_clients_r: Receiver<Message>,
    ) -> Self {
        Self {
            root: Node::new().permissions(shared::security::permissions::Permissions::Public),
            clients: HashMap::new(),
            to_server_s: to_server_s,
            from_server_r: from_server_r,
            from_clients_r,
        }
    }

    fn run(mut self) -> Result<(), Error> {
        loop {
            select_biased! {
                recv(self.from_server_r) -> msg => {
                    let quit = self.handle_server_message(Error::from(msg)?)?;

                    if quit {
                        self.quit();
                        return Ok(())
                    }
                },


                recv(self.from_clients_r) -> msg => {
                    self.handle_client_message(Error::from(msg)?)?;
                }
            };
        }
    }

    fn quit(&mut self) {
        // Quit
    }

    fn handle_client_message(&mut self, message: Message) -> Result<(), Error> {
        // A client can
        match message {
            Message::Normal(id, msg) => {
                if self.clients.contains_key(&id) {
                    self.handle_valid_client_msg(id, msg)
                } else {
                    return Err(Error::SimpleErrorStr(format!(
                        "Handler: Client {id} doesnt exist."
                    )));
                }
            }
            _ => Ok(()),
        }
    }

    fn handle_valid_client_msg(&mut self, id: u64, msg: TreeChange) -> Result<(), Error> {
        let client_sender = self.clients.get_mut(&id).unwrap(); // unwrap is safe as it was checked
        // before
        match msg {}
        Ok(())
    }

    fn handle_server_message(&mut self, message: Message) -> Result<bool, Error> {
        match message {
            Message::Register(id) => {
                // Register a new client therefore we need to create a new channel. This is a one
                // directional from handler to client.
                // Clients send their messages through
                // self.from_clients_r
                //
                let (to_handler_client_s, to_handler_client_r) =
                    crossbeam::channel::unbounded::<Message>();
                self.clients.insert(id, to_handler_client_s);
                if let Err(err) = self
                    .to_server_s
                    .send(Message::RegisterResponse(to_handler_client_r))
                {
                    return Err(Error::SimpleErrorStr(format!(
                        "Could'nt send response to client register. {:?}",
                        err
                    )));
                }

                Ok(false)
            }
            Message::Quit => Ok(true),
            _ => Err(Error::SimpleError(
                "Non server message sent to from server :c",
            )),
        }
    }
}
