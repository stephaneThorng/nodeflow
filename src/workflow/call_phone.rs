use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, NodeId, Worker};

pub struct CallPhone {
    pub phone_number: String,
}

impl CallPhone {
    pub fn new(phone_number: String) -> Self {
        Self { phone_number }
    }

    fn display(&self) {
        println!("Calling phone number: {:?}", self.phone_number);
    }
}

impl Worker for NodeId<CallPhone> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        self.module.display();
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
