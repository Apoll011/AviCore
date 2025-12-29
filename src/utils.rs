use crate::ctx::runtime;
use tokio::runtime::Handle;

#[allow(dead_code)]
pub async fn get_last_listener() -> Result<String, Box<dyn std::error::Error>> {
    match runtime().device.get_ctx("avi.dialogue.listener").await {
        Ok(v) => Ok(v.to_string()),
        Err(_) => Ok(runtime().device.get_core_id().await?)
    }
}

pub async fn get_speaker() -> Result<String, Box<dyn std::error::Error>> {
    match runtime().device.get_ctx("avi.dialogue.speaker").await {
        Ok(v) => Ok(v.to_string()),
        Err(_) => Ok(runtime().device.get_core_id().await?)
    }
}
pub fn speak(text: &str) {
    let handle = Handle::current();

    handle.block_on(async {
        let speaker = get_speaker().await.unwrap();
        runtime()
            .device
            .publish(&format!("speak/{}/text", speaker), Vec::from(text.as_bytes()))
            .await
            .unwrap();
    });
}

#[allow(dead_code)]
pub fn listen() {
    let handle = Handle::current();
    handle.block_on(async {
        let listener = get_last_listener().await.unwrap();
        runtime()
            .device
            .publish(&format!("listening/{}/start", listener), Vec::new())
            .await
            .unwrap();
    });
}