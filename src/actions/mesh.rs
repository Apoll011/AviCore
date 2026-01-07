use crate::actions::action::Action;
use crate::ctx::runtime;
use crate::subscribe;
use avi_device::device::AviDevice;
use log::{info, warn};
use std::sync::Arc;

pub struct MeshConfig {}
pub struct MeshAction {
    /// Reference to the Avi device.
    device: Arc<AviDevice>,
}
pub async fn on_peer_disconnected(avi_device: AviDevice, peer_id: String) {
    match avi_device
        .delete_ctx(&format!("avi.device.caps.{}", peer_id))
        .await
    {
        Ok(_) => info!("Peer {} removed from caps", peer_id),
        Err(e) => warn!("Error removing peer {} from caps: {}", peer_id, e),
    }

    let mut data = avi_device.get_ctx("").await.unwrap();

    if let Some(speaker) = data
        .get("avi")
        .and_then(|v| v.get("dialogue"))
        .and_then(|v| v.get("speaker"))
        .and_then(|v| v.as_str())
        && speaker == peer_id
        && let Some(avi) = data.get_mut("avi").and_then(|v| v.as_object_mut())
    {
        info!("Speaker {} removed", peer_id);
        avi.remove("speaker");
    }

    if let Some(speaker) = data
        .get("avi")
        .and_then(|v| v.get("dialogue"))
        .and_then(|v| v.get("listener"))
        .and_then(|v| v.as_str())
        && speaker == peer_id
        && let Some(avi) = data.get_mut("avi").and_then(|v| v.as_object_mut())
    {
        info!("Listener {} removed", peer_id);
        avi.remove("listener");
    }

    match data.get("avi") {
        Some(v) => avi_device.update_ctx("avi", v.clone()).await.unwrap(),
        None => warn!("No avi data, while trying to update the context."),
    }
}

pub async fn on_started(_device: AviDevice, _peer_id: String, _listening_address: Vec<String>) {
    info!("Started Avi Device.");
    if let Ok(c) = runtime() {
        c.user.get_from_disk()
    };
}

pub async fn on_peer_connected(_device: AviDevice, peer_id: String, address: String) {
    info!(
        "Connected Avi Device {} from {} into the mesh.",
        peer_id, address
    );
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(match runtime() {
            Ok(c) => c.user.save_to_device(),
            Err(_) => return,
        })
    });
}

impl Action for MeshAction {
    type Config = MeshConfig;
    fn new(_config: Self::Config) -> Result<MeshAction, String> {
        Ok(Self {
            device: Arc::clone(&runtime()?.device),
        })
    }

    async fn register(&mut self) {
        self.device.on_started(on_started).await;
        self.device.on_peer_connected(on_peer_connected).await;
        self.device.on_peer_disconnected(on_peer_disconnected).await;

        let _ = subscribe!("user/update", async: move |_from, _topic, _data| async move {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(match runtime() {
                    Ok(c) => c.user.load_from_device(),
                    Err(_) => return,
                })
            });
        });
    }
}
