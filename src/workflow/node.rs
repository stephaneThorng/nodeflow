use crate::workflow::flow::FlowState;

#[derive(Default)]
pub struct NodeId<T> {
    pub idx: usize,
    pub module_name: ModuleType,
    pub status: NodeStatus,
    pub children: Vec<usize>,
    pub module: T,
}

pub trait Worker {
    fn process(&mut self, state: &mut FlowState) -> Option<&usize> {
        self.handle(state);
        self.next(state)
    }

    fn handle(&mut self, state: &mut FlowState);
    fn next(&mut self, state: &mut FlowState) -> Option<&usize>;

    fn get_module_type(&self) -> ModuleType;
    fn get_id(&self) -> usize;
    fn add_child(&mut self, idx: usize);
}

impl<T> NodeId<T> {
    pub fn new(idx: usize, module_name: ModuleType, module: T) -> Self {
        Self {
            idx,
            module_name,
            status: NodeStatus::Started,
            children: Default::default(),
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
