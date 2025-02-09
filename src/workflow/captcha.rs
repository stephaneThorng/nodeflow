use crate::workflow::flow::FlowState;
use crate::workflow::node::{NodeStatus, Worker};
use std::thread::sleep;
use std::time::Duration;

pub struct Captcha {
    pub id: u8,
    pub valid: bool,
}

impl Captcha {
    pub fn new(id: u8) -> Self {
        Self { id, valid: false }
    }
}

impl Worker for Captcha {
    fn handle(&mut self, state: &mut FlowState) -> NodeStatus {
        println!("captcha valid ? : {:?}", self.valid);
        println!("captcha id: {:?}", self.id);
        println!("Loading...");
        sleep(Duration::from_secs(2));
        self.valid = true;
        println!("captcha valid ? : {:?}", self.valid);
        state.auth_level += 10;
        println!("Update auth_level to : {:?}", state.auth_level);
        NodeStatus::Success
    }
}
