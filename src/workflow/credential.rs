use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, NodeId, NodeStatus, Worker};
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

impl Worker for NodeId<Credential> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", "Crendtial");
        self.module.display();
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
        println!("Determine the next node to handle...");
        sleep(Duration::from_secs(3));
        if rand::random::<u8>() % 2 == 0 {
            self.status = NodeStatus::Success;
        } else {
            self.status = NodeStatus::Failed;
        }

        println!("Next node is: {:?}", self.children[0]);
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
