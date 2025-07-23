use shared::{datatypes::nodes::Node, errors::Error};
use uuid::Uuid;

pub trait ServerInterface {
    /// Add a child to the root node of the tree.
    fn add_child(&mut self, node: Node) -> Result<(), Error>;
    /// Add a child to the the node with the corresponding id.
    fn add_child_to_node(&mut self, node: Node, parent_id: &Uuid) -> Result<(), Error>;
    // get a immutable reference of the node with this id.
    fn get_node(&self, id: &Uuid) -> Result<&Node, Error>;
    // get a mutable reference of the node with this id.
    fn get_node_mut(&mut self, id: &Uuid) -> Result<&mut Node, Error>;
}
