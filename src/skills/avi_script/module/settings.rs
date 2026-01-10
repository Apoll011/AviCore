use crate::skills::avi_script::helpers::get_skill_context;
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, NativeCallContext, Position};

#[export_module]
pub mod settings_module {
    /// Gets a setting value for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the setting
    ///
    /// # Returns
    /// The setting value, or default if not set
    #[rhai_fn(global, return_raw)]
    pub fn get(ctx: NativeCallContext, name: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(
            skill_context.config.setting(&name).unwrap_or_default(),
        ))
    }

    /// Lists all settings available for the current skill
    ///
    /// # Returns
    /// A list of setting names
    #[rhai_fn(global, return_raw)]
    pub fn list(ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(skill_context.config.list_settings()))
    }

    /// Checks if a setting exists for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the setting
    ///
    /// # Returns
    /// True if the setting exists, false otherwise
    #[rhai_fn(global, return_raw)]
    pub fn has(ctx: NativeCallContext, name: String) -> Result<bool, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(skill_context.config.has_setting(&name))
    }

    /// Gets the full setting object including metadata
    ///
    /// # Arguments
    /// * `name` - The name of the setting
    ///
    /// # Returns
    /// The full setting object, or UNIT if not found
    #[rhai_fn(global, return_raw)]
    pub fn full(ctx: NativeCallContext, name: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(skill_context.config.get_setting_full(&name)))
    }
}
