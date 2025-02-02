use crate::workflow::flow::FlowState;
use crate::workflow::node::{into_next, ModuleType, Node, NodeChild, Process};
use uuid::Uuid;

pub struct Credential {
    pub username: String,
}

impl Credential {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

impl Node<Credential> {
    pub fn new(module_name: ModuleType, next: impl Process + 'static, username: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            module_name,
            next: into_next(next),
            child: Credential::new(username),
        }
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
