use shared::datatypes::nodes::Node;

extern crate client;
extern crate server;
extern crate shared;

fn main() {
    let mut server = server::Server::new();

    server.add_child(Node::new().name("Hello"));

    server.serve();
}
