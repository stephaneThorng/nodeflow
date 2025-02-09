use crate::workflow::arena::Arena;
use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::flow::{Execute, FlowState};
use crate::workflow::node::{ModuleType, NodeId, Worker};
use workflow::call_phone::CallPhone;

mod workflow;

fn main() {
    println!("Hello, world!");

    // let call_module: Node<CallPhone> = Node::new(
    //     ModuleType::PhoneNumber,
    //     HashMap::new(),
    //     CallPhone::new("+33123456678".to_string()),
    // );
    // let captcha_module: Node<Captcha> =
    //     Node::new(ModuleType::Captcha, HashMap::new(), Captcha::new(15));
    //
    // let mut next: HashMap<NodeStatus, Box<dyn Worker>> = HashMap::new();
    // next.insert(NodeStatus::Success, Box::new(captcha_module));
    // next.insert(NodeStatus::Failed, Box::new(call_module));
    // let login_module: Node<Credential> = Node::new(
    //     ModuleType::LoginPassword,
    //     next,
    //     Credential::new("admin".to_string()),
    // );
    //
    //
    //
    // let mut config = FlowConfig::new(Box::new(login_module));

    // let mut state = FlowState::default();
    //
    // config.start(&mut state);

    let mut arena: Arena = Arena::new();
    let mut credential_node = Box::new(NodeId::new(
        0,
        ModuleType::LoginPassword,
        Credential::new("admin".to_string()),
    ));
    let mut call_node = Box::new(NodeId::new(
        1,
        ModuleType::PhoneNumber,
        CallPhone::new("+33123456678".to_string()),
    ));
    let mut captcha_node = Box::new(NodeId::new(2, ModuleType::Captcha, Captcha::new(15)));

    let credential_id = arena.add_node(credential_node);
    let call_id = arena.add_node(call_node);
    let captcha_id = arena.add_node(captcha_node);

    arena.nodes[credential_id].add_child(call_id);
    arena.nodes[call_id].add_child(captcha_id);

    let mut state = FlowState::default();
    arena.start(&mut state);
}
