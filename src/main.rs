#[macro_use]
extern crate dyon;

mod skills;
mod intent;
mod api;
mod ctx;

use std::sync::Arc;
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use avi_device::DeviceCapabilities;
use tokio::sync::Mutex;
use crate::api::api::Api;
use crate::ctx::RuntimeContext;
use crate::ctx::RUNTIMECTX;
use crate::skills::manager::SkillManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = RUNTIMECTX.set(RuntimeContext {
        api_url: "http://127.0.0.1:1178".into(),
        lang: "pt".into(),
        skill_path: "./skills".into(),
    });

    let api = Arc::new(Mutex::new(Api::new()));
    let manager = Arc::new(Mutex::new(SkillManager::new()));

    let config = AviDeviceConfig {
        node_name: "avi-core".to_string(),
        device_type: AviDeviceType::CORE,
        can_gateway_embedded: false,
        capabilities: DeviceCapabilities::default(),
    };

    let device = AviDevice::new(config).await?;

    let d_clone = device.clone();
    tokio::spawn(async move {
        d_clone.start_event_loop().await;
    });

    let api_clone = Arc::clone(&api);
    let manager_clone = Arc::clone(&manager);

    device.subscribe_async("intent/execute/text", move |_from, _topic, data| {
        let api = Arc::clone(&api_clone);
        let manager = Arc::clone(&manager_clone);

        async move {
            let msg = String::from_utf8_lossy(&data);

            let maybe_intent = match api.lock().await.intent(&*msg).await {
                Ok(intent) => Some(intent),
                Err(e) => {
                    println!("Failed to parse intent: {}", e);
                    None
                }
            };

            if let Some(intent) = maybe_intent {
                let mut mg = manager.lock().await;
                if let Err(e) = mg.run_intent(intent) {
                    println!("Error executing intent: {}", e);
                }
            }
        }    }).await.map_err(|e| e.to_string())?;

    loop {

    }

    #[allow(unreachable_code)]
    Ok(())
}
