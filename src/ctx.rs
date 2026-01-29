use crate::data::config::ConfigSystem;
use crate::data::context::ContextManager;
use crate::data::user::UserManager;
use crate::dialogue::languages::LanguageSystem;
use crate::dialogue::reply::{ReplyConfig, ReplyManager};
use avi_device::device::AviDevice;
use log::{debug, error, info, trace};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tokio::runtime::Handle;

/// Holds the runtime configuration and shared resources for the AviCore application.
pub struct RuntimeContext {
    /// A handle to the Tokio runtime for spawning async tasks.
    pub rt: Handle,
    /// A shared reference to the Avi device.
    pub device: Arc<AviDevice>,

    pub config_path: PathBuf,

    pub reply_manager: ReplyManager,

    pub language_system: LanguageSystem,

    pub configuration: ConfigSystem,

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

pub fn create_runtime(config_path: &str, device: Arc<AviDevice>) {
    trace!("Creating runtime with config_path={}", config_path);
    info!("Initializing runtime.");
    RUNTIMECTX
        .set(Arc::from(RuntimeContext {
            device,
            rt: Handle::current(),
            reply_manager: ReplyManager::new(Option::from(ReplyConfig {
                timeout_secs: 30,
                max_retries: Some(3),
            })),
            language_system: LanguageSystem::new(&format!("{}/lang", config_path)),
            configuration: ConfigSystem::new(&format!("{}/config", config_path)),
            context: ContextManager::new(format!("{}/context", config_path)),
            user: UserManager::new(),
            config_path: config_path.into(),
        }))
        .unwrap_or_else(|_| {
            error!("Failed to set Runtime Context: already initialized.");
            panic!("Runtime context already initialized")
        });
    info!("Runtime initialized successfully.");
}
