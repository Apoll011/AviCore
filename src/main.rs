mod actions;
mod api;
mod config;
mod context;
mod ctx;
mod dialogue;
mod log;
mod macros;
pub mod skills;
mod user;
mod utils;

use crate::actions::action::Action;
use crate::actions::dialogue::{DialogueAction, DialogueCapability};
use crate::actions::intent::IntentAction;
use crate::actions::mesh::MeshAction;
use crate::config::setting_or;
use crate::context::context_cleanup_task;
use crate::ctx::{create_runtime, runtime};
use crate::log::AviCoreLogger;
use ::log::{error, info};
use avi_device::DeviceCapabilities;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use clap::Parser;
use std::sync::Arc;
use std::time::Duration;
use crate::utils::generate_documentation;

#[derive(Parser, Debug)]
#[command(name = "AviCore")]
#[command(about = "AviCore application", long_about = None)]
struct Args {
    /// Generate documentation and exit
    #[arg(long = "generate-docs")]
    generate_docs: bool,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    AviCoreLogger::init();

    let args = Args::parse();

    if args.generate_docs {
        return generate_documentation();
    }

    info!("Starting the System");

    let config_path = "./config";
    let config = AviDeviceConfig {
        node_name: "avi-core".to_string(),
        device_type: AviDeviceType::CORE,
        can_gateway_embedded: false,
        capabilities: DeviceCapabilities::default(),
    };

    let device = Arc::new(AviDevice::new(config).await?);

    device.start_event_loop();

    create_runtime(config_path, device);

    register_action!(IntentAction, {
        watch_skill_dir: setting_or::<bool>("watch_skill_dir", false),
        watch_dir_debounce_time: setting_or::<u64>("watch_dir_debounce_time", 1),
    });

    register_action!(DialogueAction, {
        capability: DialogueCapability::new(setting_or::<String>("dialogue_cap", "both".to_string())),
    });

    register_action!(MeshAction);

    watch_dir!(&format!("{}/config", config_path), Duration::from_secs(1), async: |_event| {
        match runtime() {
            Ok(v) => {
                info!("Change in config directory. Reloading Configuration.");
                v.configuration.reload()
            },
            Err(e) => error!("Error reloading configuration: {}", e),
        }
    });

    context_cleanup_task();

    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}