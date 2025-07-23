use std::rc::Rc;

use server::server_interface::ServerInterface;
use shared::{
    datatypes::{Data, nodes::Node},
    events::DataChanged,
};

extern crate client;
extern crate server;
extern crate shared;

fn main() {
    let mut server = server::Server::new();

    server.add_child(Node::new().name("Hello")).unwrap();
    let mut n = Node::new();

    n.subscribe_to_children(DataChanged::new(move |old, new| {
        println!("Hello {}, {}", old.data, new);
    }));
    n.change_data(Data::UInt32(32));
}
