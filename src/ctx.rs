use crate::context::ContextManager;
use crate::dialogue::languages::LanguageSystem;
use crate::dialogue::reply::ReplyManager;
use avi_device::device::AviDevice;
use std::sync::{Arc, OnceLock};
use tokio::runtime::Handle;

/// Holds the runtime configuration and shared resources for the AviCore application.
pub struct RuntimeContext {
    /// The base URL for the Avi API.
    pub(crate) api_url: String,
    /// The language setting for the application (e.g., "pt", "en").
    pub(crate) lang: String,
    /// The filesystem path where skills are located.
    pub(crate) skill_path: String,
    /// A handle to the Tokio runtime for spawning async tasks.
    pub rt: Handle,
    /// A shared reference to the Avi device.
    pub device: Arc<AviDevice>,

    pub reply_manager: ReplyManager,

    pub language_system: LanguageSystem,

    pub context: ContextManager,
}

/// Global static storage for the `RuntimeContext`.
///
/// It uses a `OnceLock` to ensure that the context is initialized exactly once.
pub(crate) static RUNTIMECTX: OnceLock<Arc<RuntimeContext>> = OnceLock::new();

/// Provides global access to the `RuntimeContext`.
///
/// # Panics
///
/// Panics if the runtime context has not been initialized yet.
///
/// TODO: Consider returning an `Option` or `Result` instead of panicking, or provide a non-panicking alternative.
pub fn runtime() -> &'static Arc<RuntimeContext> {
    RUNTIMECTX.get().expect("Runtime not initialized")
}
