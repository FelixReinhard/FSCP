use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    datatypes::Data, errors::Error, events::EventSubscriber, security::permissions::Permissions,
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

    /// Sets the nodes id.
    /// This is only needed when reconstruing a tree.
    /// Normally, a new id is generated when adding a node.
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    /// Changes the data of this node.
    /// Should be used at runtime after the tree has been configured. When configuring use [Node::data]
    ///
    /// Triggers the [DataChanged] event
    pub fn change_data(&mut self, data: Data) {
        let old_data = self.data.clone();
        self.data = data;

        if let Some(subs) = &self.subscribers {
            if let Data::Button(_) = self.data {
                for s in subs {
                    s.handle_button_press(&self);
                }
            } else {
                for s in subs {
                    s.handle_data_changed(&self, &old_data);
                }
            }
        }
    }

    /// Changes the name of this node.
    /// Should be used at runtime after the tree has been configured. When configuring use
    /// [Node::name]
    ///
    /// Triggers the [NameChanged] event
    pub fn change_name(&mut self, name: impl Display) {
        let old_name = self.name.clone();
        self.name = Some(name.to_string());

        if let Some(subs) = &self.subscribers {
            for s in subs {
                s.handle_name_changed(&self, &old_name);
            }
        }
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
    pub fn get_child(&mut self, index: usize) -> Result<&mut Node, Error> {
        if let Some(c) = &mut self.children {
            if let Some(child) = c.get_mut(index) {
                return Ok(child);
            }
        }
        Err(Error::SimpleError("This child does not exist"))
    }

    /// Traverses the tree to find a node with the given id.
    pub fn find_node_mut(&mut self, id: &Uuid) -> Option<&mut Node> {
        if self.id == *id {
            return Some(self);
        }

        if let Some(children) = &mut self.children {
            for child in children {
                if let Some(node) = child.find_node_mut(id) {
                    return Some(node);
                }
            }
        }

        None
    }

    /// Deletes the node with id in the tree.
    /// Returns [true] if this was successfull, [false] otherwise.
    ///
    /// It is impossible to delete the root.
    pub fn remove_child(&mut self, id: &Uuid) -> bool {
        if let Some(children) = &mut self.children {
            let old_size = children.len();
            children.retain(|elem| {
                if elem.id == *id {
                    // Found => Should be deleted
                    //
                    // trigger deleted event.
                    elem.trigger_deleted();
                    false
                } else {
                    false
                }
            });

            // difference should never be greater then one as Uuid ids are unique.
            if old_size != children.len() {
                return true;
            }
        }
        false
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

    /// Subscribe to this node might emit.
    ///
    /// Possible Events:
    /// - [DataChanged]
    /// - [ChildAdded]
    /// - [ChildRemoved]
    /// - [NameChanged]
    /// - [ButtonPressed]
    ///
    /// # Example:
    ///
    /// ```
    /// use shared::datatypes::nodes::Node;
    /// use shared::events::DataChanged;
    ///
    /// let mut node = Node::new();
    ///
    /// node.subscribe(DataChanged::new(|_, _| {
    ///    println!("Hello");
    /// }
    /// ));
    /// ```
    pub fn subscribe(&mut self, subscriber: impl EventSubscriber + 'static) {
        if let Some(s) = &mut self.subscribers {
            s.push(Box::new(subscriber));
        } else {
            self.subscribers = Some(vec![Box::new(subscriber)]);
        }
    }

    /// Subscribe to this node and all its children might emit.
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

    /// Hashes the tree using
    /// - id
    /// - name
    /// - data
    /// - children
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Press this node. Only for [Data::Button].
    /// Should **not** be used on the *Server*, only on the *Client*
    ///
    /// No [ChangeData] event is triggered, but a [ButtonPress]
    pub fn press(&mut self) -> Result<(), Error> {
        if let Data::Button(n) = self.data {
            // Increase amount of pressed and use [change_data] to also trigger the events.

            self.change_data(Data::Button(n + 1));
            Ok(())
        } else {
            Err(Error::SimpleErrorStr(format!(
                "Press: Node is not a button ({:?})",
                self.id
            )))
        }
    }

    /// Used internally to trigger the deletion event.
    fn trigger_deleted(&self) {
        // First trigger self
        if let Some(subs) = &self.subscribers {
            for s in subs {
                s.handle_child_removed(&self);
            }
        }
        // then trigger children
        if let Some(children) = &self.children {
            for c in children {
                c.trigger_deleted();
            }
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.data.hash(state);

        if let Some(children) = &self.children {
            for child in children {
                child.hash(state);
            }
        }
    }
}
