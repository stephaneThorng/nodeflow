use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, Node, Worker};

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

impl Worker for Node<CallPhone> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        self.module.display();
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
    }

    fn next(&mut self, state: &mut FlowState) -> Option<&mut Box<dyn Worker>> {
        self.next.get_mut(&self.status)
    }

    fn get_module_type(&self) -> ModuleType {
        self.module_name
    }
}
