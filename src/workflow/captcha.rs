use crate::workflow::arena::ArenaState;
use crate::workflow::node::{Module, NodeStatus};
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

impl Module for Captcha {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus {
        println!("\tcaptcha valid ? : {:?}", self.valid);
        println!("\tcaptcha id: {:?}", self.id);
        println!("\tLoading...");
        sleep(Duration::from_secs(2));
        self.valid = true;
        println!("\tcaptcha valid ? : {:?}", self.valid);
        state.auth_level += 10;
        println!("\tUpdate auth_level to : {:?}", state.auth_level);
        NodeStatus::Success
    }
}
