use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, NodeId, Worker};
use std::thread::sleep;
use std::time::Duration;

pub struct Captcha {
    pub id: u8,
    pub valid: bool,
}

impl Captcha {
    pub fn new(id: u8) -> Self {
        Self { id, valid: false }
    }
}

impl Worker for NodeId<Captcha> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        println!("captcha valid ? : {:?}", self.module.valid);
        println!("captcha id: {:?}", self.module.id);
        println!("Loading...");
        sleep(Duration::from_secs(2));
        self.module.valid = true;
        println!("captcha valid ? : {:?}", self.module.valid);
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
    }

    fn next(&mut self, state: &mut FlowState) -> Option<&usize> {
        self.children.get(0)
    }

    fn get_module_type(&self) -> ModuleType {
        self.module_name
    }

    fn get_id(&self) -> usize {
        self.idx
    }

    fn add_child(&mut self, idx: usize) {
        self.children.push(idx);
    }
}
