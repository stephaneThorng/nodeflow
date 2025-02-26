use crate::workflow::arena::ArenaState;
use crate::workflow::node::{Module, NodeStatus};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

static MODULE_CAPTCHA_SETTED: &str = "MODULE_CAPTCHA_SETTED";

#[derive(Serialize, Deserialize)]
pub struct Captcha {
    pub id: u8,
    pub valid: bool,
}

impl Captcha {
    pub fn new(id: u8) -> Self {
        Self { id, valid: false }
    }
}

#[typetag::serde]
impl Module for Captcha {
    fn handle(&mut self, state: &mut ArenaState) -> NodeStatus {
        // Arena status is ArenaStatus.UserInput
        println!("\tSetting captcha...");

        let sss = "MODULE_CAPTCHA_SETTED".to_string();
        let status = match &state.data.get("step") {
            Some(sss) => {
                let code_value = state.data.get("captcha_code_value");
                println!(
                    "\tcaptcha valid ? expected {:?}, got {:?}",
                    state.data.get("captcha_code"),
                    code_value
                );
                // Check if the captcha is valid
                if state.data.get("captcha_code") == code_value {
                    println!("\tCaptcha is valid !");
                    state.auth_level += 10;
                    println!("\tUpdate auth_level to : {:?}", state.auth_level);
                    return NodeStatus::Success;
                } else {
                    println!("\tCaptcha is invalid !");
                    return NodeStatus::Failed;
                }
            }
            _ => {
                println!("\tCaptcha not setted yet ...");
                let random_string: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);
                println!("\tGenerated new captcha: {}", random_string);
                state.data.insert("captcha_code".to_string(), random_string);
                NodeStatus::Step(MODULE_CAPTCHA_SETTED.to_string())
            }
        };

        status
    }
}
