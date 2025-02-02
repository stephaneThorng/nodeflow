use crate::workflow::flow::FlowState;
use uuid::Uuid;

pub trait NodeChild {}
#[derive(Default)]
pub struct Node<T: NodeChild> {
    pub id: Uuid,
    pub module_name: ModuleType,
    pub next: Option<Box<dyn Process>>,
    pub child: T,
}

pub trait Process {
    fn execute(&mut self, state: &mut FlowState) {
        self.handle(state);

        if let Some(next) = self.next(state) {
            next.execute(state);
        }
    }

    fn handle(&mut self, state: &mut FlowState);
    fn next(&mut self, state: &mut FlowState) -> &mut Option<Box<dyn Process>>;
}

impl<T: NodeChild> Node<T> {
    pub fn new(module_name: ModuleType, next: Option<Box<dyn Process>>, child: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            module_name,
            next,
            child,
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
