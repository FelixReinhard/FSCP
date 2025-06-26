use crate::datatypes::{Data, nodes::Node};

/// This trait implements the events.
/// A event can be subscribed to, where other clients and even the server don't get a notification
/// about which events where subscribed to.
///
///
pub trait EventSubscriber {
    /// Is called when the data of a node changes in any way.
    fn handle_data_changed(&self, node: &Node, previous_data: &Data) {}
    /// This is called before the node is added. Therefore, the children of [node] do not contain
    /// [new_child].
    fn handle_child_added(&self, node: &Node, new_child: &Node) {}

    fn handle_child_removed(&self, node: &Node) {}

    fn handle_name_changed(&self, node: &Node, previous_name: &Option<String>) {}

    /// Special event that is triggered when a node is the [Data::Button] and is pressed.
    ///
    /// This event is transmitted, only from client to server.
    /// It should be used to let the client trigger changes or behaviour.
    fn handle_button_press(&self, node: &Node) {}
}

/// This macro creates the framework to easily create new event types
/// and add them easily.
macro_rules! make_event_subscriber {
    ($struct_name:ident, $fn_trait: path, $implemetation: item) => {
        pub struct $struct_name<F>
        where
            F: $fn_trait + Clone,
        {
            handler: F,
        }

        impl<F> $struct_name<F>
        where
            F: $fn_trait + Clone,
        {
            pub fn new(handler: F) -> Self {
                Self { handler }
            }
        }

        impl<F> EventSubscriber for $struct_name<F>
        where
            F: $fn_trait + Clone,
        {
        $implemetation
        }

        impl<F> Clone for $struct_name<F> where F: $fn_trait + Clone {
            fn clone(&self) -> Self {
                Self {handler: self.handler.clone()}
            }
        }

    };
}

make_event_subscriber!(
    DataChanged,
    Fn(&Node, &Data),
    fn handle_data_changed(&self, node: &Node, previous_data: &Data) {
        (self.handler)(node, previous_data)
    }
);

make_event_subscriber!(
    ChildAdded,
    Fn(&Node, &Node),
    fn handle_child_added(&self, node: &Node, new_child: &Node) {
        (self.handler)(node, new_child)
    }
);

make_event_subscriber!(
    NameChanged,
    Fn(&Node, &Option<String>),
    fn handle_name_changed(&self, node: &Node, previous_name: &Option<String>) {
        (self.handler)(node, previous_name)
    }
);

make_event_subscriber!(
    ChildRemoved,
    Fn(&Node),
    fn handle_child_removed(&self, node: &Node) {
        (self.handler)(node)
    }
);

make_event_subscriber!(
    ButtonPressed,
    Fn(&Node),
    fn handle_button_press(&self, node: &Node) {
        (self.handler)(node)
    }
);
