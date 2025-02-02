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

pub fn into_next(node: impl Process + Sized + 'static) -> Option<Box<dyn Process>> {
    Some(Box::new(node))
}

// impl<T: NodeChild> Node<T> {
//     pub fn new(module_name: ModuleType, next: impl Process + 'static) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             module_name,
//             next: into_next(next),
//             child: T,
//         }
//     }
// }

#[derive(Default, Debug)]
pub enum ModuleType {
    #[default]
    LoginPassword,
    Captcha,
}

// pub trait NodeChild {}
// pub struct NodeRunner<T: NodeChild> {
//     pub node: Rc<Node>,
//     pub state: HashMap<String, String>,
//     pub min_auth_level: u8,
//     pub child: T
// }

// impl NodeRunner {
//     pub fn new(name: NodeType, min_auth_level: u8) -> Self {
//         Self {
//             node: Rc::new(Node {
//                 name,
//                 // flow,
//                 previous: None,
//                 next: HashMap::new(),
//                 input: HashMap::new(),
//             }),
//             state: HashMap::new(),
//             min_auth_level,
//
//         }
//     }
// }
//
//
// pub trait ProcessTrait<T:NodeChild> {
//     fn process(&self) -> Result<NodeStatus, &'static str>;
// }
//
//
//
// pub trait ExecuteTrait<T:NodeChild>: ProcessTrait<T> {
//     fn execute(&self) -> Result<NodeStatus, &'static str> {
//         if self.can_execute() {
//             self.process()
//         } else {
//             Err("Min auth level not reached")
//         }
//     }
//
//     fn can_execute(&self) -> bool {
//         // self.get_node().node.flow.auth_level >= self.get_node().min_auth_level
//         println!("can_execute ...");
//         true
//     }
//     fn get_node(&self) -> &NodeRunner<T> {
//         &self
//     }
// }

pub enum NodeStatus {
    Success,
    Failed,
    Ignored,
}
