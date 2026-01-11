use crate::actions::action::Action;
use crate::actions::dialogue::{DialogueAction, DialogueCapability};
use crate::actions::intent::IntentAction;
use crate::actions::mesh::MeshAction;
use crate::config::setting_or;
use crate::context::context_cleanup_task;
use crate::ctx::{create_runtime, runtime};
use crate::{register_action, watch_dir};
use avi_device::DeviceCapabilities;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;

pub async fn start_avi(
    is_core: bool,
    can_gate_away: bool,
    config_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting the System");

    let config = AviDeviceConfig {
        node_name: "avi-core".to_string(),
        device_type: if is_core {
            AviDeviceType::CORE
        } else {
            AviDeviceType::NODE
        },
        can_gateway_embedded: can_gate_away,
        capabilities: DeviceCapabilities::default(),
    };

    let device = Arc::new(AviDevice::new(config).await?);

    device.start_event_loop();

    create_runtime(&config_path, device);

    register_action!(DialogueAction, {
        capability: DialogueCapability::new(setting_or::<String>("dialogue_cap", "both".to_string())),
    });

    register_action!(IntentAction, if: is_core, {
        watch_skill_dir: setting_or::<bool>("watch_skill_dir", false),
        watch_dir_debounce_time: setting_or::<u64>("watch_dir_debounce_time", 1),
    });

    register_action!(MeshAction, if: is_core);

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
