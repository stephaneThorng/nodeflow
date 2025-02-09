use crate::workflow::flow::FlowState;
use crate::workflow::node::{NodeStatus, Worker};
use std::thread::sleep;
use std::time::Duration;

pub struct Credential {
    pub username: String,
}

impl Credential {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    fn display(&self) {
        println!("username: {:?}", self.username);
    }
}

impl Worker for Credential {
    fn handle(&mut self, state: &mut FlowState) -> NodeStatus {
        println!("Handling node: {:?}", "Crendential");
        self.display();
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
        println!("Determine the next node to handle...");
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
