use std::sync::Arc;
use avi_device::device::AviDevice;
use crate::actions::action::Action;
use crate::ctx::runtime;

pub struct MeshConfig {}
pub struct MeshAction {
    /// Reference to the Avi device.
    device: Arc<AviDevice>,
}

pub async fn on_peer_disconnected(
    avi_device: AviDevice,
    peer_id: String,
) {
   match avi_device.delete_ctx(&format!("avi.device.caps.{}", peer_id)).await {
        Ok(_) => println!("Peer {} removed from caps", peer_id),
        Err(e) => println!("Error removing peer {} from caps: {}", peer_id, e),
    }
}

impl Action for MeshAction {
    type Config = MeshConfig;
    fn new(_config: Self::Config) -> Self {
        Self {
            device: Arc::clone(&runtime().device),
        }
    }

    async fn register(&mut self) {
        self.device.on_peer_disconnected(on_peer_disconnected).await;
    }
}