use shared::{
    datatypes::{Data, nodes::Node},
    events::{ChildAdded, DataChanged},
};

extern crate client;
extern crate server;
extern crate shared;

fn main() {
    let mut server = server::Server::new();

    server.add_child(Node::new().name("Hello"));
    let mut n = Node::new();
    n.subscribe_to_children(DataChanged::new(|node, data| {
        println!("Hello");
    }));
    n.change_data(Data::UInt32(32));

    server.serve();
}
