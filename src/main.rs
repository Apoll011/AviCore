#[macro_use]
extern crate dyon;

mod skills;
mod intent;
mod api;
mod ctx;
mod actions;

use std::sync::Arc;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use avi_device::DeviceCapabilities;
use tokio::sync::Mutex;
use crate::actions::action::Action;
use crate::actions::dialogue::{DialogueAction, DialogueCapability, DialogueConfig};
use crate::actions::intent::{IntentAction, IntentConfig};
use crate::api::api::Api;
use crate::ctx::RuntimeContext;
use crate::ctx::RUNTIMECTX;
use crate::skills::manager::SkillManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    RUNTIMECTX.set(RuntimeContext {
        api_url: "http://127.0.0.1:1178".into(),
        lang: "pt".into(),
        skill_path: "./skills".into(),
    }).expect("Failed to set runtime context");

    let api = Arc::new(Mutex::new(Api::new()));
    let manager = Arc::new(Mutex::new(SkillManager::new()));

    let config = AviDeviceConfig {
        node_name: "avi-core".to_string(),
        device_type: AviDeviceType::CORE,
        can_gateway_embedded: false,
        capabilities: DeviceCapabilities::default(),
    };

    let device = Arc::new(AviDevice::new(config).await?);

    let device_clone = Arc::clone(&device);
    tokio::spawn(async move {
        device_clone.start_event_loop().await;
    });

    let mut intent_action = IntentAction::new(&device, IntentConfig {
        api: Arc::clone(&api),
        skill_manager: Arc::clone(&manager),
    });
    intent_action.register().await;

    let mut dialogue_action = DialogueAction::new(&device, DialogueConfig {
        capability: DialogueCapability::SPEAKER
    });
    dialogue_action.register().await;

    tokio::signal::ctrl_c().await?;
    println!("Shutting down gracefully...");

    Ok(())
}