use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

// pub struct FlowConfig {
//     pub id: Uuid,
//     pub root_node: Box<dyn Worker>,
// }
//
// impl FlowConfig {
//     pub fn new(root_node: Box<dyn Worker>) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             root_node,
//         }
//     }
// }

#[derive(Default)]
pub struct FlowState {
    pub id: Uuid,
    pub status: FlowStatus,
    pub state: HashMap<String, String>,
    pub auth_level: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Default)]
pub enum FlowStatus {
    #[default]
    Started,
    Failed,
    Ended,
}

pub trait Execute {
    fn start(&mut self, state: &mut FlowState);
}
//
// impl Execute for FlowConfig {
//     fn start(&mut self, state: &mut FlowState) {
//         println!("flow_id = {:?}", state.id);
//         self.root_node.process(state);
//     }
// }
