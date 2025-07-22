use crossbeam::channel::Receiver;
use shared::remote::message::Message;

#[derive(Debug)]
pub(crate) enum InternalMessage {
    Message(u64, Message),
    Register(u64),
    RegisterResponse(u64, Receiver<InternalMessage>),
    Quit,
}
