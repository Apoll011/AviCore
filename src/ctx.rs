use crate::context::ContextManager;
use crate::dialogue::languages::LanguageSystem;
use crate::dialogue::reply::{ReplyConfig, ReplyManager};
use crate::user::UserManager;
use avi_device::device::AviDevice;
use log::{debug, error, info, trace};
use std::sync::{Arc, OnceLock};
use tokio::runtime::Handle;

/// Holds the runtime configuration and shared resources for the AviCore application.
pub struct RuntimeContext {
    /// The base URL for the Avi API.
    pub api_url: String,
    /// The language setting for the application (e.g., "pt", "en").
    pub lang: String,
    /// The filesystem path where skills are located.
    pub skill_path: String,
    /// A handle to the Tokio runtime for spawning async tasks.
    pub rt: Handle,
    /// A shared reference to the Avi device.
    pub device: Arc<AviDevice>,

    pub reply_manager: ReplyManager,

    pub language_system: LanguageSystem,

    pub context: ContextManager,

    pub user: UserManager,
}

/// Global static storage for the `RuntimeContext`.
///
/// It uses a `OnceLock` to ensure that the context is initialized exactly once.
pub static RUNTIMECTX: OnceLock<Arc<RuntimeContext>> = OnceLock::new();

/// Provides global access to the `RuntimeContext`.
///
/// # Panics
///
/// Panics if the runtime context has not been initialized yet.
pub fn runtime() -> Result<&'static Arc<RuntimeContext>, String> {
    match RUNTIMECTX.get() {
        Some(runtime) => Ok(runtime),
        None => {
            debug!("Attempted to access unitialized Runtime Context");
            Err("Runtime Context not initialized".to_string())
        }
    }
}

pub fn create_runtime(api_url: &str, lang: &str, config_path: &str, device: Arc<AviDevice>) {
    trace!(
        "Creating runtime with api_url={}, lang={}, config_path={}",
        api_url, lang, config_path
    );
    info!("Initializing runtime.");
    RUNTIMECTX
        .set(Arc::from(RuntimeContext {
            api_url: api_url.to_string(),
            lang: lang.to_string(),
            skill_path: format!("{}/skills", config_path),
            device,
            rt: Handle::current(),
            reply_manager: ReplyManager::new(Option::from(ReplyConfig {
                timeout_secs: 30,
                max_retries: Some(3),
            })),
            language_system: LanguageSystem::new(&format!("{}/lang", config_path)),
            context: ContextManager::new(format!("{}/context", config_path)),
            user: UserManager::new(),
        }))
        .unwrap_or_else(|_| {
            error!("Failed to set Runtime Context: already initialized.");
            panic!("Runtime context already initialized")
        });
    info!("Runtime initialized successfully.");
}
