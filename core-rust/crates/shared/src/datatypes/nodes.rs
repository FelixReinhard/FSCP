use std::{collections::HashMap, fmt::Display};

use crate::{
    datatypes::Data,
    errors::{self, Error},
    events::{EventSubscriber, EventType},
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

    subscribers: Option<Vec<Box<dyn EventSubscriber>>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            data: Data::Folder,
            children: None,
            name: None,
            id: Uuid::new_v4(),
            permissions: Permissions::All,
            subscribers: None,
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
    ///
    /// Should only be used when creating a node.
    /// For runtime changes use [change_data].
    pub fn data(mut self, data: Data) -> Self {
        self.data = data;
        self
    }

    pub fn change_data(&mut self, data: Data) {
        if let Some(subs) = &self.subscribers {
            for s in subs {
                s.handle_data_changed(&self, &self.data);
            }
        }
        self.data = data;
    }

    /// Adds a single new node to the children list.
    pub fn add_child(&mut self, node: Node) -> &mut Self {
        // trigger the event.
        if let Some(subs) = &self.subscribers {
            for s in subs {
                s.handle_child_added(&self, &node);
            }
        }

        // Add the node to the children list.
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

    /// Subscribe to an event this node might emit.
    ///
    /// Possible Events:
    /// - [DataChanged]
    /// - [ChildAdded]
    pub fn subscribe(&mut self, subscriber: impl EventSubscriber + 'static) {
        if let Some(s) = &mut self.subscribers {
            s.push(Box::new(subscriber));
        } else {
            self.subscribers = Some(vec![Box::new(subscriber)]);
        }
    }

    /// Subscribe to an event this node and all its children might emit.
    ///
    /// Possible Events:
    /// - [DataChanged]
    /// - [ChildAdded]
    pub fn subscribe_to_children(&mut self, subscriber: impl EventSubscriber + 'static + Clone) {
        if let Some(ch) = &mut self.children {
            for child in ch {
                child.subscribe_to_children(subscriber.clone());
            }
        }

        self.subscribe(subscriber);
    }
}
