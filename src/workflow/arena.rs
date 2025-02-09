use crate::workflow::flow::FlowState;
use crate::workflow::node::Worker;

pub struct Arena {
    pub nodes: Vec<Box<dyn Worker>>,
}

impl Arena {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn get_node(&mut self, idx: usize) -> &Box<dyn Worker> {
        self.nodes.get_mut(idx).unwrap()
    }

    pub fn add_node(&mut self, node: Box<dyn Worker>) -> usize {
        let idx = node.get_id().clone();
        self.nodes.push(node);
        idx
    }

    pub(crate) fn start(&mut self, state: &mut FlowState) {
        println!("flow_id = {:?}", state.id);

        let mut next = self.get_node(0);

        while let Some(idx) = next.process(state) {
            next = self.get_node(*idx);
        }
    }
}
