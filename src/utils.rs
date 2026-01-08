use crate::ctx::runtime;
use log::warn;

pub async fn core_id() -> Option<String> {
    match runtime() {
        Ok(c) => match c.device.get_core_id().await {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("Error getting core id: {}", e.to_string());
                None
            }
        },
        Err(_) => None,
    }
}
