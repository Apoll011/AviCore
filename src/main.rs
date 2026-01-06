#[macro_use]
extern crate dyon;

mod actions;
mod api;
mod config;
mod context;
mod ctx;
mod dialogue;
mod macros;
mod skills;
mod user;

use crate::actions::action::Action;
use crate::actions::dialogue::{DialogueAction, DialogueCapability};
use crate::actions::intent::{IntentAction};
use crate::actions::mesh::{MeshAction};
use crate::context::context_cleanup_task;
use crate::ctx::create_ctx;
use avi_device::DeviceCapabilities;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use std::sync::Arc;

/// Entry point for the AviCore application.
///
/// This function initializes the device, set up the runtime context,
/// and registers actions like intent handling and dialogue management.
///
/// # Errors
///
/// Returns an error if device initialization, context setup, or signal handling fails.
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

    register_action!(IntentAction);

    register_action!(DialogueAction, {
        capability: DialogueCapability::Speaker,
    });

    register_action!(MeshAction);

    context_cleanup_task();

    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}
