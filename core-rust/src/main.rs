use std::rc::Rc;

use shared::{
    datatypes::{Data, nodes::Node},
    events::DataChanged,
};

extern crate client;
extern crate server;
extern crate shared;

fn main() {
    let num = Rc::new(42);
    let mut server = server::Server::new();

    server.add_child(Node::new().name("Hello"));
    let mut n = Node::new();

    let num2 = num.clone();
    n.subscribe_to_children(DataChanged::new(move |_, _| {
        println!("Hello {num2}");
    }));
    n.change_data(Data::UInt32(32));

    server.serve();
}
