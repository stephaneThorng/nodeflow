use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::flow::{FlowConfig, FlowState, FlowTrait};
use crate::workflow::node::{ModuleType, Node};

mod workflow;

fn main() {
    println!("Hello, world!");

    let captcha_module: Node<Captcha> = Node::newCaptcha(ModuleType::Captcha, 15);
    let login_module: Node<Credential> = Node::new(
        ModuleType::LoginPassword,
        captcha_module,
        "admin".to_string(),
    );
    let mut config = FlowConfig::new(Box::new(login_module));

    let mut state = FlowState::default();

    config.execute(&mut state);

    // let login_password = LoginPasswordModule::new(ModuleType::LoginPassword, 0);
    // login_password.execute();
}
