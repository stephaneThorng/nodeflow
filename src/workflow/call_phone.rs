use crate::workflow::arena::ArenaState;
use crate::workflow::node::{NodeStatus, Module};

pub struct CallPhone {
    pub phone_number: String,
}

impl CallPhone {
    pub fn new(phone_number: String) -> Self {
        Self { phone_number }
    }

    fn display(&self) {
        println!("\tCalling phone number: {:?}", self.phone_number);
    }
}

impl Module for CallPhone {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus {
        self.display();
        state.auth_level += 10;
        println!("\tUpdate auth_level to : {:?}", state.auth_level);
        NodeStatus::Success
    }
}
