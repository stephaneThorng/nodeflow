use crate::workflow::flow::FlowState;
use crate::workflow::node::{ModuleType, NodeId, Worker};

pub struct Arena<T: Worker> {
    pub nodes: Vec<NodeId<T>>,
}

impl<T: Worker> Arena<T> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn get_node(&mut self, idx: usize) -> &mut NodeId<T> {
        self.nodes.get_mut(idx).unwrap()
    }

    pub fn add_node(&mut self, module_name: ModuleType, module: T) -> usize {
        let idx = self.nodes.len();
        let node = NodeId::new(2, module_name, module);
        self.nodes.push(node);
        idx
    }

    pub(crate) fn start(&mut self, state: &mut FlowState) {
        println!("flow_id = {:?}", state.id);

        // let mut next = self.get_node(0);
        //
        // while let Some(idx) = next.process(state) {
        //     next = self.get_node(*idx);
        // }
    }
}
