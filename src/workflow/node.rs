use crate::workflow::arena::ArenaState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub idx: usize,
    pub children: HashMap<NodeStatus, usize>,
    pub module: Box<dyn Module>,
}

#[typetag::serde(tag = "type")]
pub trait Module {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus;
}

impl Node {
    pub fn new(idx: usize, module: Box<dyn Module>) -> Self {
        Self {
            idx,
            children: Default::default(),
            module,
        }
    }

    pub(crate) fn add_child(&mut self, node_status: NodeStatus, idx: usize) {
        self.children.insert(node_status, idx);
    }

    pub(crate) fn process(&mut self, state: &mut ArenaState) -> Option<&usize> {
        println!("Handling node: {:?}", self.module.typetag_name());
        sleep(Duration::from_secs(1));
        let status = self.module.handle(state);
        println!("This node return code : {:?}", status);
        if let NodeStatus::Step(step) = status.clone() {
            state.data.insert("step".to_string(), step);
        }

        self.next(status)
    }

    fn next(&mut self, status: NodeStatus) -> Option<&usize> {
        self.children.get(&status)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ModuleType {
    #[default]
    LoginPassword,
    Captcha,
    PhoneNumber,
}

#[derive(Default, Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub enum NodeStatus {
    #[default]
    Started,
    Success,
    Failed,
    Ignored,
    Step(String),
}
