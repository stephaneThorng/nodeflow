use crate::workflow::arena::ArenaState;
use crate::workflow::node::{Module, NodeStatus};
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
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

#[typetag::serde]
impl Module for Credential {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus {
        self.display();
        state.auth_level += 10;
        println!("\tUpdate auth_level to : {:?}", state.auth_level);
        println!("\tDetermine the next node to handle...");
        sleep(Duration::from_secs(0));

        NodeStatus::Success
    }
}
