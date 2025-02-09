use crate::workflow::flow::FlowState;

#[derive(Default)]
pub struct NodeId<T: Worker> {
    pub idx: usize,
    pub module_name: ModuleType,
    pub status: NodeStatus,
    pub children: Vec<usize>,
    pub module: T,
}

pub trait Worker {
    fn handle(&mut self, state: &mut FlowState) -> NodeStatus;
}

impl<T: Worker > NodeId<T> {
    pub fn new(idx: usize, module_name: ModuleType, module: T) -> Self {
        Self {
            idx,
            module_name,
            status: NodeStatus::Started,
            children: Default::default(),
            module,
        }
    }

    pub(crate) fn add_child(&mut self, idx: usize) {
        self.children.push(idx);
    }

    fn process(&mut self, state: &mut FlowState) -> Option<&usize> {
        println!("Handling node: {:?}", self.module_name);
        self.module.handle(state);
        println!("This node return code : {:?}", self.status);
        self.next()
    }

    fn next(&mut self) -> Option<&usize> {
        self.children.get(0)
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
