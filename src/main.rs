use crate::workflow::arena::Arena;
use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::flow::{FlowState};
use crate::workflow::node::{ModuleType, Worker};
use workflow::call_phone::CallPhone;

mod workflow;

fn main() {
    println!("Hello, world!");

    let mut arena: Arena<dyn Worker> = Arena::new();

    let credential_id = arena.add_node(
        ModuleType::LoginPassword,
        Credential::new("admin".to_string()),
    );
    let call_id = arena.add_node(
        ModuleType::PhoneNumber,
        CallPhone::new("+33123456678".to_string()),
    );
    let captcha_id = arena.add_node(ModuleType::Captcha, Captcha::new(15));

    arena.nodes[credential_id].add_child(call_id);
    arena.nodes[call_id].add_child(captcha_id);

    let mut state = FlowState::default();
    arena.start(&mut state);
}
