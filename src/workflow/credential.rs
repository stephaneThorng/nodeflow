use crate::workflow::flow::FlowState;
use crate::workflow::node::{Node, NodeChild, Process};

pub struct Credential {
    pub username: String,
}

impl Credential {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

impl NodeChild for Credential {}

impl Process for Node<Credential> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        println!("username: {:?}", self.child.username);
    }

    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Process>> {
        &mut self.next
    }
}
