use crossbeam::channel::{Receiver, Sender};
use shared::datatypes::treebuilder::TreeChange;

pub enum Message {
    Normal(u64, TreeChange),
    Quit,
    Register(u64),
    RegisterResponse(Receiver<Message>),
}
