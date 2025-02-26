use crate::workflow::node::{Module, Node};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

static MODULE_START: &str = "MODULE_START";
pub(crate) static CURRENT_NODE: &str = "CURRENT_NODE";

#[derive(Serialize, Deserialize)]
pub struct Arena {
    pub id: Uuid,
    pub nodes: Vec<Node>,
    pub state: ArenaState,
}

impl Arena {
    pub fn new(state: ArenaState) -> Self {
        Self {
            id: Uuid::new_v4(),
            nodes: vec![],
            state,
        }
    }

    pub fn get_node(&mut self, idx: usize) -> &mut Node {
        self.nodes.get_mut(idx).unwrap()
    }

    pub fn add_node(&mut self, module: Box<dyn Module>) -> usize {
        let idx = self.nodes.len();
        let node = Node::new(idx, module);
        self.nodes.push(node);
        idx
    }

    pub(crate) fn start(&mut self, start_node_idx: usize) {
        println!("Starting arena id {:?}", self.state.id);

        let mut next_node_idx = start_node_idx;
        let state = &mut self.state;

        while let Some(idx) = {
            let node_idx = next_node_idx;
            let node = self.nodes.get_mut(node_idx).unwrap();
            node.process(state)
        } {
            next_node_idx = *idx;
        }
        self.state
            .data
            .insert(CURRENT_NODE.to_string(), next_node_idx.to_string());
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ArenaState {
    pub id: Uuid,
    pub status: ArenaStatus,
    pub data: HashMap<String, String>,
    pub auth_level: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Default, Serialize, Deserialize)]
pub enum ArenaStatus {
    #[default]
    Started,
    Failed,
    Ended,
    UserInput(usize, String, HashMap<String, Input>), // Node_idx who represent the module and the next action inside the module
}

#[derive(Serialize, Deserialize)]
pub enum Input {
    Text(String),
    InputString(String), // value
    InputNumber(usize),
    InputChoice(Vec<String>, String),
}

impl ArenaState {
    pub fn new(created_by: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: ArenaStatus::Started,
            data: Default::default(),
            auth_level: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by,
        }
    }
}
