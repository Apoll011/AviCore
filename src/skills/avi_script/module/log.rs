use crate::skills::avi_script::helpers::get_skill_name;
use log::{debug, error, info, trace, warn};
use rhai::plugin::*;
use rhai::NativeCallContext;

#[export_module]
pub mod log_module {
    /// Logs an informational message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    pub fn info(ctx: NativeCallContext, text: ImmutableString) {
        match get_skill_name(&ctx) {
            Ok(v) => info!("Skill {} - {}", v, text),
            Err(_) => {
                warn!(
                    "Skill - Could not get skill name, logging as info: {}",
                    text
                )
            }
        };
    }

    /// Logs a trace-level message for detailed debugging
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    pub fn trace(ctx: NativeCallContext, text: ImmutableString) {
        match get_skill_name(&ctx) {
            Ok(v) => trace!("Skill {} - {}", v, text),
            Err(_) => warn!(
                "Skill - Could not get skill name, logging as trace: {}",
                text
            ),
        };
    }

    /// Logs a debug-level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    pub fn debug(ctx: NativeCallContext, text: ImmutableString) {
        match get_skill_name(&ctx) {
            Ok(v) => debug!("Skill {} - {}", v, text),
            Err(_) => warn!(
                "Skill - Could not get skill name, logging as debug: {}",
                text
            ),
        };
    }

    /// Logs a warning message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    pub fn warn(ctx: NativeCallContext, text: ImmutableString) {
        match get_skill_name(&ctx) {
            Ok(v) => warn!("Skill {} - {}", v, text),
            Err(_) => warn!(
                "Skill - Could not get skill name, logging as warn: {}",
                text
            ),
        };
    }

    /// Logs an error message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    pub fn error(ctx: NativeCallContext, text: ImmutableString) {
        match get_skill_name(&ctx) {
            Ok(v) => error!("Skill {} - {}", v, text),
            Err(_) => warn!(
                "Skill - Could not get skill name, logging as info: error: {}",
                text
            ),
        };
    }
}
