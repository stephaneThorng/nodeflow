use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, Node, NodeChild, Process};
use uuid::Uuid;

pub struct Captcha {
    pub id: u8,
}

impl Captcha {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl NodeChild for Captcha {}

impl Process for Node<Captcha> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        println!("captcha id: {:?}", self.child.id);
    }

    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Process>> {
        &mut self.next
    }
}
