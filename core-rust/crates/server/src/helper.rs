use crossbeam::channel::{Receiver, Sender};
use shared::errors::Error;

use crate::internal_message::InternalMessage;

pub(crate) struct ServerHelper {
    to_handler_s: Sender<InternalMessage>,
    from_handler_r: Receiver<InternalMessage>,
    to_handler_from_clients_s: Sender<InternalMessage>,
    client_id: u64, // starts at 0 and is increased for each new client.
}

impl ServerHelper {
    pub fn new(
        to_handler_s: Sender<InternalMessage>,
        from_handler_r: Receiver<InternalMessage>,
        to_handler_from_clients_s: Sender<InternalMessage>,
    ) -> Self {
        Self {
            to_handler_s,
            from_handler_r,
            to_handler_from_clients_s,
            client_id: 0,
        }
    }

    // Give a new client the to_handler_from_clients_s cloned. This is the communication client ->
    // handler,
    // Then we send with to_handler_s has higher priority than to_handler_from_clients_s.
    // With response registering is done and we get a reciever, that connects to the handler
    // directily.
    pub fn register(
        &mut self,
    ) -> Result<(Sender<InternalMessage>, Receiver<InternalMessage>), Error> {
        let new_id = self.client_id;
        self.client_id += 1;

        // Send a register to the handler
        Error::from(self.to_handler_s.send(InternalMessage::Register(new_id)))?;
        let register_response = Error::from(self.from_handler_r.recv())?;

        match register_response {
            InternalMessage::RegisterResponse(id, reciever) => {
                if id != new_id {
                    return Err(Error::SimpleError("Tried registering but wrong id"));
                }

                Ok((self.to_handler_from_clients_s.clone(), reciever))
            }
            e => Err(Error::SimpleErrorStr(format!(
                "Tried registering but didnt get a response got {:?}",
                e
            ))),
        }
    }
}
