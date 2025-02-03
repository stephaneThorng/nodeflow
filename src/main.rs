use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::flow::{Execute, FlowConfig, FlowState};
use crate::workflow::node::{ModuleType, Node};

mod workflow;

fn main() {
    println!("Hello, world!");

    let captcha_module: Node<Captcha> = Node::new(ModuleType::Captcha, None, Captcha::new(15));
    let login_module: Node<Credential> = Node::new(
        ModuleType::LoginPassword,
        Some(Box::new(captcha_module)),
        Credential::new("admin".to_string()),
    );
    let mut config = FlowConfig::new(Box::new(login_module));

    let mut state = FlowState::default();

    config.start(&mut state);
}
