use shared::{datatypes::nodes::Node, errors::Error, security::permissions::Permissions};

pub struct Server {
    root: Node,
}

impl Server {
    pub fn new() -> Self {
        Self {
            root: Node::new().name("root").permissions(Permissions::Public),
        }
    }

    /// Starts the server and serves the configured tree.
    pub fn serve(&mut self) {}

    /// Add a child to the root node of the tree.
    pub fn add_child(&mut self, node: Node) {
        self.root.add_child(node);
    }

    /// Get the nth child of the root node.
    /// If not present returns an error.
    pub fn get_child(&mut self, index: usize) -> Result<&mut Node, Error> {
        self.root.get_child(index)
    }

    /// Get the amount of children the root node has.
    pub fn get_child_count(&self) -> usize {
        if let Some(c) = &self.root.children {
            c.len()
        } else {
            0
        }
    }
}
