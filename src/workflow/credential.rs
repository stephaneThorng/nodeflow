use crate::workflow::flow::FlowState;
use crate::workflow::node::{Node, Worker};

pub struct Credential {
    pub username: String,
}

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
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
    }

    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Worker>> {
        &mut self.next
    }
}
