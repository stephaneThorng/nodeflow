use crate::workflow::flow::FlowState;
use crate::workflow::node::{Module, Node, Worker};

pub struct Credential {
    pub username: String,
}
impl Module for Credential {}

impl Credential {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    fn display(&self) {
        println!("username: {:?}", self.username);
    }
}

impl Worker for Node<Credential> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        self.module.display();
    }

    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Worker>> {
        &mut self.next
    }
}
