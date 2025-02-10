use crate::workflow::node::{Module, ModuleType, NodeId};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Arena {
    pub nodes: Vec<NodeId>,
}

impl Arena {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn get_node(&mut self, idx: usize) -> &mut NodeId {
        self.nodes.get_mut(idx).unwrap()
    }

    pub fn add_node(&mut self, module_name: ModuleType, module: Box<dyn Module>) -> usize {
        let idx = self.nodes.len();
        let node = NodeId::new(2, module_name, module);
        self.nodes.push(node);
        idx
    }

    pub(crate) fn start(&mut self, state: &mut ArenaState) {
        println!("Starting arena id {:?}", state.id);

        let mut next_node_idx = 0;

        while let Some(idx) = {
            let node = self.get_node(next_node_idx);
            node.process(state)
        } {
            next_node_idx = *idx;
        }
    }
}

#[derive(Default)]
pub struct ArenaState {
    pub id: Uuid,
    pub status: ArenaStatus,
    pub state: HashMap<String, String>,
    pub auth_level: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Default)]
pub enum ArenaStatus {
    #[default]
    Started,
    Failed,
    Ended,
}
