use std::collections::HashMap;

use crossbeam::channel::{Receiver, Sender, select_biased};
use shared::{datatypes::nodes::Node, errors::Error, remote::message::Message};

use crate::internal_message::InternalMessage;

// Handles the managment of the tree.
pub(crate) struct ServerHandler {
    root: Node,
    /// Map that connects an client id to a sending channel
    clients: HashMap<u64, Sender<InternalMessage>>,
    to_server_s: Sender<InternalMessage>,
    from_server_r: Receiver<InternalMessage>,
    from_clients_r: Receiver<InternalMessage>,
}

impl ServerHandler {
    pub fn new(
        to_server_s: Sender<InternalMessage>,
        from_server_r: Receiver<InternalMessage>,
        from_clients_r: Receiver<InternalMessage>,
    ) -> Self {
        Self {
            root: Node::new().permissions(shared::security::permissions::Permissions::Public),
            clients: HashMap::new(),
            to_server_s: to_server_s,
            from_server_r: from_server_r,
            from_clients_r,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        loop {
            select_biased! {
                recv(self.from_server_r) -> msg => {
                    // let quit = self.handle_server_message(Error::from(msg)?)?;
                    //
                    // if quit {
                    //     self.quit();
                    //     return Ok(())
                    // }
                },
                recv(self.from_clients_r) -> msg => {
                   // self.handle_client_message(Error::from(msg)?)?;
                }
            };
        }
    }

    fn quit(&mut self) {
        // Quit
    }

    fn handle_msg(&mut self, msg: InternalMessage) {}
}
