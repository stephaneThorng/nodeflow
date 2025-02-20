use crate::workflow::node::{NodeStatus, Module};
use std::thread::sleep;
use std::time::Duration;
use crate::workflow::arena::ArenaState;

pub struct Credential {
    pub username: String,
}

impl Credential {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    fn display(&self) {
        println!("\tusername: {:?}", self.username);
    }
}

impl Module for Credential {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus {
        self.display();
        state.auth_level += 10;
        println!("\tUpdate auth_level to : {:?}", state.auth_level);
        println!("\tDetermine the next node to handle...");
        sleep(Duration::from_secs(3));
        let status: NodeStatus;
        if rand::random::<u8>() % 2 == 0 {
            status = NodeStatus::Success;
        } else {
            status = NodeStatus::Failed;
        }

        status
    }
}
