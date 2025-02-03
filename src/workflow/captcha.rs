use crate::workflow::flow::FlowState;
use crate::workflow::node::{Module, Node, Worker};

pub struct Captcha {
    pub id: u8,
}

impl Captcha {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Module for Captcha {}

impl Worker for Node<Captcha> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        println!("captcha id: {:?}", self.module.id);
    }

    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Worker>> {
        &mut self.next
    }
}
