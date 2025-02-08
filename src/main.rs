use std::collections::HashMap;
use workflow::call_phone::CallPhone;

use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::flow::{Execute, FlowConfig, FlowState};
use crate::workflow::node::{ModuleType, Node, NodeStatus, Worker};

mod workflow;

fn main() {
    println!("Hello, world!");

    let call_module: Node<CallPhone> = Node::new(
        ModuleType::PhoneNumber,
        HashMap::new(),
        CallPhone::new("+33123456678".to_string()),
    );
    let captcha_module: Node<Captcha> =
        Node::new(ModuleType::Captcha, HashMap::new(), Captcha::new(15));

    let mut next: HashMap<NodeStatus, Box<dyn Worker>> = HashMap::new();
    next.insert(NodeStatus::Success, Box::new(captcha_module));
    next.insert(NodeStatus::Failed, Box::new(call_module));
    let login_module: Node<Credential> = Node::new(
        ModuleType::LoginPassword,
        next,
        Credential::new("admin".to_string()),
    );
    let mut config = FlowConfig::new(Box::new(login_module));

    let mut state = FlowState::default();

    config.start(&mut state);
}
