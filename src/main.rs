#[macro_use]
extern crate dyon;

mod actions;
mod api;
mod config;
mod context;
mod ctx;
mod dialogue;
mod skills;
mod user;

use crate::actions::action::Action;
use crate::actions::dialogue::{DialogueAction, DialogueCapability, DialogueConfig};
use crate::actions::intent::{IntentAction, IntentConfig};
use crate::actions::mesh::{MeshAction, MeshConfig};
use crate::context::context_cleanup_task;
use crate::ctx::create_ctx;
use avi_device::DeviceCapabilities;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use std::sync::Arc;

/// Entry point for the AviCore application.
///
/// This function initializes the device, setup the runtime context,
/// and registers actions like intent handling and dialogue management.
///
/// # Errors
///
/// Returns an error if device initialization, context setup, or signal handling fails.
///
/// TODO: Consider moving hardcoded configuration values (API URL, paths) to a configuration file or environment variables.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AviDeviceConfig {
        node_name: "avi-core".to_string(),
        device_type: AviDeviceType::CORE,
        can_gateway_embedded: false,
        capabilities: DeviceCapabilities::default(),
    };

    let device = Arc::new(AviDevice::new(config).await?);

    device.start_event_loop();

    create_ctx("http://localhost:1178", "pt", "./config", device);

    let mut intent_action = IntentAction::new(IntentConfig {});
    intent_action.register().await;

    let mut dialogue_action = DialogueAction::new(DialogueConfig {
        capability: DialogueCapability::Speaker,
    });
    dialogue_action.register().await;

    let mut mesh_action = MeshAction::new(MeshConfig {});
    mesh_action.register().await;

    context_cleanup_task();

    tokio::signal::ctrl_c().await?;
    println!("Shutting down gracefully...");

    Ok(())
}
