use serde::{Deserialize, Serialize};
use crate::workflow::arena::ArenaState;
use crate::workflow::node::{NodeStatus, Module};

#[derive(Serialize, Deserialize)]
pub struct CallPhone {
    pub phone_number: String,
}

impl CallPhone {
    pub fn new(phone_number: String) -> Self {
        Self { phone_number }
    }

    fn display(&self) {
        println!("\tCalling phone number: {:?}", self.phone_number);
        println!("\tCalling phone number: {:?}", self.phone_number);
    }
}

#[typetag::serde]
impl Module for CallPhone {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus {
        self.display();


        state.auth_level += 10;
        println!("\tUpdate auth_level to : {:?}", state.auth_level);
        let status: NodeStatus;
        if rand::random::<u8>() % 2 == 0 {
            status = NodeStatus::Success;
        } else {
            status = NodeStatus::Failed;
        }

        status
    }
}
