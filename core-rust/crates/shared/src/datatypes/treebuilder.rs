// A tree builder accepts messages and builts and changes the tree with these messages.
// Makes it easier in the future to recieve tcp messages to build the tree.
//
//
//

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    datatypes::{Data, nodes::Node},
    errors::Error,
};

/// Defines possible changes that can be done to the tree.
#[derive(Serialize, Deserialize, Debug)]
pub enum TreeChange {
    NodeAdded(Data, Option<String>, Uuid, Uuid), // Data of added node, name of node Id of parent.
    NodeRemoved(Uuid),
    NodeChangedName(Uuid, String),
    NodeChangedData(Uuid, Data),
}

pub struct TreeBuilder;

impl TreeBuilder {
    pub fn change(root: &mut Node, change: TreeChange) -> Result<u64, Error> {
        match change {
            // A node has been added.
            TreeChange::NodeAdded(data, Some(name), id, parent) => {
                if let Some(parent) = root.find_node_mut(&parent) {
                    parent.add_child(Node::new().name(name).data(data).id(id));
                } else {
                    return Err(Error::SimpleErrorStr(format!(
                        "TreeBuilder: Cannot find node with id={:?}",
                        parent
                    )));
                }
            }
            TreeChange::NodeAdded(data, None, id, parent) => {
                if let Some(parent) = root.find_node_mut(&parent) {
                    parent.add_child(Node::new().data(data).id(id));
                } else {
                    return Err(Error::SimpleErrorStr(format!(
                        "TreeBuilder: Cannot find node with id={:?}",
                        parent
                    )));
                }
            }

            TreeChange::NodeRemoved(id) => {
                if !root.remove_child(&id) {
                    return Err(Error::SimpleError("No node to delete"));
                }
            }

            // Data has changed.
            TreeChange::NodeChangedData(id, data) => {
                if let Some(node) = root.find_node_mut(&id) {
                    node.change_data(data);
                } else {
                    return Err(Error::SimpleErrorStr(format!(
                        "TreeBuilder: Cannot find node with id={:?}",
                        id
                    )));
                }
            }

            // Name has changed.
            TreeChange::NodeChangedName(id, name) => {
                if let Some(node) = root.find_node_mut(&id) {
                    node.change_name(name);
                } else {
                    return Err(Error::SimpleErrorStr(format!(
                        "TreeBuilder: Cannot find node with id={:?}",
                        id
                    )));
                }
            }
        };

        Ok(root.get_hash())
    }
}

#[cfg(test)]
pub mod test {
    use uuid::Uuid;

    use crate::datatypes::{
        Data,
        nodes::Node,
        treebuilder::{TreeBuilder, TreeChange},
    };

    fn make_default_tree() -> Node {
        let right = Node::new()
            .name("Hello")
            .data(Data::Folder)
            .id(Uuid::from_u128(0));
        let left = Node::new()
            .name("World")
            .data(Data::Int32(64))
            .id(Uuid::from_u128(42));
        Node::new()
            .children(vec![right, left])
            .name("root")
            .id(Uuid::from_u128(1337))
    }

    #[test]
    fn simple1() {
        let mut tree = make_default_tree();
        let mut tree2 = make_default_tree();

        let id = tree2.get_child(0).unwrap().id.clone();
        let id2 = tree2.get_child(1).unwrap().id.clone();

        tree.get_child(0)
            .unwrap()
            .change_name("Hello and Good Morning");
        tree.get_child(1).unwrap().change_data(Data::Bool(true));

        TreeBuilder::change(
            &mut tree2,
            TreeChange::NodeChangedName(id, "Hello and Good Morning".to_string()),
        )
        .unwrap();

        TreeBuilder::change(
            &mut tree2,
            TreeChange::NodeChangedData(id2, Data::Bool(true)),
        )
        .unwrap();

        assert_eq!(tree.get_hash(), tree2.get_hash());
    }
    #[test]
    fn simple2() {
        let mut tree = make_default_tree();
        let mut tree2 = make_default_tree();

        for i in 0..100 {
            tree.add_child(Node::new().name(format!("node {i}")).id(Uuid::from_u128(i)));
        }

        let root_id = tree2.id.clone();
        for i in 0..100 {
            TreeBuilder::change(
                &mut tree2,
                TreeChange::NodeAdded(
                    Data::Folder,
                    Some(format!("node {i}")),
                    Uuid::from_u128(i),
                    root_id.clone(),
                ),
            )
            .unwrap();
        }

        assert_eq!(tree.get_hash(), tree2.get_hash());
    }

    #[test]
    fn remove() {
        let mut tree = make_default_tree();
        let mut tree2 = make_default_tree();

        let right_id = tree.get_child(0).unwrap().id.clone();
        tree.remove_child(&right_id);

        TreeBuilder::change(&mut tree2, TreeChange::NodeRemoved(right_id)).unwrap();

        assert_eq!(tree.get_hash(), tree2.get_hash());
    }
}
