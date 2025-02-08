use std::collections::HashMap;
use crate::workflow::flow::FlowState;
use uuid::Uuid;

#[derive(Default)]
pub struct Node<T> {
    pub id: Uuid,
    pub module_name: ModuleType,
    pub status: NodeStatus,
    pub next: HashMap<NodeStatus, Box<dyn Worker>>,
    pub module: T,
}

pub trait Worker {
    fn process(&mut self, state: &mut FlowState) {
        self.handle(state);

        if let Some(mut next) = self.next(state) {
            next.process(state);
        }
    }

    fn handle(&mut self, state: &mut FlowState);
    fn next(&mut self, state: &mut FlowState) -> Option<&mut Box<dyn Worker>>;

    fn get_module_type(&self) -> ModuleType;
}

impl<T> Node<T> {
    pub fn new(module_name: ModuleType, next: HashMap<NodeStatus, Box<dyn Worker>>, module: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            module_name,
            status: NodeStatus::Started,
            next,
            module,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum ModuleType {
    #[default]
    LoginPassword,
    Captcha,
    PhoneNumber,
}

#[derive(Default, Hash, Eq, PartialEq)]
pub enum NodeStatus {
    #[default]
    Started,
    Success,
    Failed,
    Ignored,
}


