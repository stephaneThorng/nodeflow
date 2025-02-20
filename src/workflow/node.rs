use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use crate::workflow::arena::ArenaState;

pub struct NodeId {
    pub idx: usize,
    pub module_name: ModuleType,
    pub children: HashMap<NodeStatus, usize>,
    pub module: Box<dyn Module>,
}

pub trait Module {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus;
}

impl NodeId {
    pub fn new(idx: usize, module_name: ModuleType, module: Box<dyn Module>) -> Self {
        Self {
            idx,
            module_name,
            children: Default::default(),
            module,
        }
    }

    pub(crate) fn add_child(&mut self, node_status: NodeStatus, idx: usize) {
        self.children.insert(node_status, idx);
    }

    pub(crate) fn process(&mut self, state: &mut ArenaState) -> Option<&usize> {
        println!("Handling node: {:?}", self.module_name);
        sleep(Duration::from_secs(1));
        let status = self.module.handle(state);
        println!("This node return code : {:?}", status);
        self.next(status)
    }

    fn next(&mut self, status: NodeStatus) -> Option<&usize> {
        self.children.get(&status)
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum ModuleType {
    #[default]
    LoginPassword,
    Captcha,
    PhoneNumber,
}

#[derive(Default, Hash, Eq, PartialEq, Debug)]
pub enum NodeStatus {
    #[default]
    Started,
    Success,
    Failed,
    Ignored,
}
