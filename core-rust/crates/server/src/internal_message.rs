use crossbeam::channel::Receiver;
use shared::{datatypes::treebuilder::TreeChange, remote::message::Message};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) enum InternalMessage {
    TreeChange(TreeChange),
    Message(u64, Message),
    Register(u64),
    RegisterResponse(u64, Receiver<InternalMessage>),
    Quit,

    IdResponse(Uuid),
}
