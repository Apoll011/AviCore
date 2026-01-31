use crate::actions::action::Action;
use crate::actions::dialogue::{DialogueAction, DialogueCapability};
use crate::actions::intent::IntentAction;
use crate::actions::mesh::MeshAction;
use crate::cli::setup::Setup;
use crate::cli::ui;
use crate::ctx::{create_runtime, runtime};
use crate::data::config::setting_or;
use crate::data::context::context_cleanup_task;
use crate::{register_action, watch_dir};
use avi_device::DeviceCapabilities;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use log::{error, info};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

pub async fn start_avi(config_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting the System");

    let mut setup = Setup::new(&config_path);

    setup.check().await.map(|_| info!("Setup Finished"))?;

    ui::step(3, 7, "Initializing Device Configuration");

    let is_core = setting_or::<String>("device_type", "core".to_string()).eq("core");

    let config = AviDeviceConfig {
        node_name: "avi-core".to_string(),
        device_type: if is_core {
            AviDeviceType::CORE
        } else {
            AviDeviceType::NODE
        },
        can_gateway_embedded: setting_or("can_gateway", false),
        capabilities: DeviceCapabilities::default(),
    };

    let device = Arc::new(AviDevice::new(config).await?);

    device.start_event_loop();

    ui::step(4, 8, "Initializing Runtime");
    create_runtime(&config_path.display().to_string(), device);

    setup
        .post_runtime_init()
        .await
        .unwrap_or_else(|e| error!("Online Setup Failed: {}", e));

    ui::step(5, 8, "Initializing Actions");

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_style(ui::spinner_style());

    pb.enable_steady_tick(Duration::from_millis(120));

    register_action!(IntentAction, pb, if: is_core, {
        watch_skill_dir: setting_or::<bool>("watch_skill_dir", false),
        watch_dir_debounce_time: setting_or::<u64>("watch_dir_debounce_time", 1),
    });

    register_action!(DialogueAction, pb, {
        capability: DialogueCapability::new(setting_or::<String>("dialogue_cap", "none".to_string())),
    });

    register_action!(MeshAction, pb, if: is_core);

    pb.finish_with_message("Actions Loaded...");

    ui::step(6, 8, "Setting the config directory watcher");
    watch_dir!(&config_path.join("config").display(), Duration::from_secs(1), async: |_event| {
        match runtime() {
            Ok(v) => {
                info!("Change in config directory. Reloading Configuration.");
                v.configuration.reload()
            },
            Err(e) => error!("Error reloading configuration: {}", e),
        }
    });

    ui::step(7, 8, "Creating context clenup task");
    context_cleanup_task();

    ui::step(8, 8, "Started AVI");
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}
