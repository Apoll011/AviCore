use std::sync::{Arc, OnceLock};
use avi_device::device::AviDevice;

pub struct RuntimeContext {
    pub(crate) api_url: String,
    pub(crate) lang: String,
    pub(crate) skill_path: String,
    pub device: Arc<AviDevice>,
}

pub(crate) static RUNTIMECTX: OnceLock<Arc<RuntimeContext>> = OnceLock::new();

pub fn runtime() -> &'static Arc<RuntimeContext> {
    RUNTIMECTX.get().expect("Runtime not initialized")
}
