use std::thread::sleep;
use std::time::Duration;
use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, Node, NodeStatus, Worker};

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

impl Worker for Node<Credential> {
    fn handle(&mut self, state: &mut FlowState) {
        println!("Handling node: {:?}", self.module_name);
        self.module.display();
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
        println!("Determine the next node to handle...");
        sleep(Duration::from_secs(3));
        if rand::random::<u8>() % 2 == 0  {
            self.status = NodeStatus::Success;
        } else {
            self.status = NodeStatus::Failed;
        }

        println!("Next node is: {:?}", self.next.get(&self.status).unwrap().get_module_type());
    }

    fn next(&mut self, state: &mut FlowState) -> Option<&mut Box<dyn Worker>> {
        self.next.get_mut(&self.status)
    }

    fn get_module_type(&self) -> ModuleType {
        self.module_name
    }
}
