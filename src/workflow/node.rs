use crate::workflow::flow::FlowState;
use uuid::Uuid;

pub trait Module {}
#[derive(Default)]
pub struct Node<T: Module> {
    pub id: Uuid,
    pub module_name: ModuleType,
    pub next: Option<Box<dyn Worker>>,
    pub module: T,
}

pub trait Worker {
    fn process(&mut self, state: &mut FlowState) {
        self.handle(state);

        if let Some(next) = self.next(state) {
            next.process(state);
        }
    }

    fn handle(&mut self, state: &mut FlowState);
    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Worker>>;
}

impl<T: Module> Node<T> {
    pub fn new(module_name: ModuleType, next: Option<Box<dyn Worker>>, module: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            module_name,
            next,
            module,
        }
    }
}

#[derive(Default, Debug)]
pub enum ModuleType {
    #[default]
    LoginPassword,
    Captcha,
}

pub enum NodeStatus {
    Success,
    Failed,
    Ignored,
}
