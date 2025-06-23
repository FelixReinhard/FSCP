use std::fmt::Display;

use crate::{
    datatypes::Data,
    errors::{self, Error},
    security::permissions::Permissions,
};
use uuid::Uuid;

/// The base type for a node.
///
pub struct Node {
    pub data: Data,

    pub name: Option<String>,
    pub id: Uuid,
    pub children: Option<Vec<Node>>,
    pub permissions: Permissions,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            data: Data::Folder,
            children: None,
            name: None,
            id: Uuid::new_v4(),
            permissions: Permissions::All,
        }
    }
}

impl Node {
    /// Creates a new Node object with default values.
    pub fn new() -> Self {
        Node::default()
    }

    // Sets the Display name of the node.
    pub fn name(mut self, name: impl Display) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the permisions of this node.
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Sets all children.
    pub fn children(mut self, children: Vec<Node>) -> Self {
        self.children = Some(children);
        self
    }

    /// Sets the nodes data. This includes its type.
    /// By default data is [Data::Folder]
    pub fn data(mut self, data: Data) -> Self {
        self.data = data;
        self
    }

    /// Adds a single new node to the children list.
    pub fn add_child(&mut self, node: Node) -> &mut Self {
        if let Some(c) = &mut self.children {
            c.push(node);
        } else {
            self.children = Some(vec![node]);
        }
        self
    }

    /// Get the child by index.
    /// Returns a mutable reference to the child if present.
    /// Otherwise returns an Error
    pub fn get_child(&mut self, index: usize) -> Result<&mut Node, errors::Error> {
        if let Some(c) = &mut self.children {
            if let Some(child) = c.get_mut(index) {
                return Ok(child);
            }
        }
        Err(Error::SimpleError("This child does not exist"))
    }

    /// Returns the amount of children this node has.
    /// If no children are present returns 0.
    pub fn get_children_count(&self) -> usize {
        if let Some(c) = &self.children {
            return c.len();
        } else {
            return 0;
        }
    }
}
