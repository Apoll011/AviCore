use crate::skills::avi_script::helpers::get_skill_name;
use log::{debug, error, info, trace, warn};
use rhai::plugin::*;
use rhai::{EvalAltResult, NativeCallContext};

#[export_module]
pub mod log_module {
    /// Logs an informational message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn info(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
        let skill_name = get_skill_name(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        info!("Skill {} - {}", skill_name, text);
        Ok(())
    }

    /// Logs a trace-level message for detailed debugging
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn trace(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
        let skill_name = get_skill_name(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        trace!("Skill {} - {}", skill_name, text);
        Ok(())
    }

    /// Logs a debug-level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn debug(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
        let skill_name = get_skill_name(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        debug!("Skill {} - {}", skill_name, text);
        Ok(())
    }

    /// Logs a warning message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn warn(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
        let skill_name = get_skill_name(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        warn!("Skill {} - {}", skill_name, text);
        Ok(())
    }

    /// Logs an error message
    ///
    /// # Arguments
    /// * `message` - The message to log
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn error(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
        let skill_name = get_skill_name(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        error!("Skill {} - {}", skill_name, text);
        Ok(())
    }
}
