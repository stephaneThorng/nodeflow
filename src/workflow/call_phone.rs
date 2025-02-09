use crate::workflow::flow::FlowState;
use crate::workflow::node::{NodeStatus, Worker};

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

impl Worker for CallPhone {
    fn handle(&mut self, state: &mut FlowState) -> NodeStatus {
        self.display();
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
        NodeStatus::Success
    }
}
