use crate::workflow::arena::{Arena, ArenaState};
use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::node::{ModuleType, NodeStatus};
use workflow::call_phone::CallPhone;

mod workflow;

fn main() {
    println!("Hello, world!");

    let mut arena = Arena::new();

    let credential_id = arena.add_node(
        ModuleType::LoginPassword,
        Box::new(Credential::new("admin".to_string())),
    );
    let call_id = arena.add_node(
        ModuleType::PhoneNumber,
        Box::new(CallPhone::new("+33123456678".to_string())),
    );
    let captcha_id = arena.add_node(ModuleType::Captcha, Box::new(Captcha::new(15)));

    arena.nodes[credential_id].add_child(NodeStatus::Success, call_id);
    arena.nodes[call_id].add_child(NodeStatus::Success, captcha_id);

    let mut state = ArenaState::default();
    arena.start(&mut state);
}
