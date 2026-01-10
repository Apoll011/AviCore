use crate::skills::avi_script::helpers::get_skill_context;
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, NativeCallContext};

#[export_module]
pub mod skill_module {
    /// Gets the root directory of the current skill
    ///
    /// # Returns
    /// The path to the skill's root directory
    #[rhai_fn(return_raw)]
    pub fn dir(ctx: NativeCallContext) -> Result<String, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(skill_context.path.clone())
    }

    /// Gets the version of the current skill
    ///
    /// # Returns
    /// The version string of the skill
    #[rhai_fn(return_raw)]
    pub fn version(ctx: NativeCallContext) -> Result<String, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(skill_context.info.version.clone())
    }

    /// Gets the manifest information of the current skill
    ///
    /// # Returns
    /// A map containing the skill's manifest
    #[rhai_fn(return_raw)]
    pub fn manifest(ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(skill_context.info.clone()))
    }

    /// Gets the permissions required by the current skill
    ///
    /// # Returns
    /// A list of permissions
    #[rhai_fn(return_raw)]
    pub fn get_permissions(ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(skill_context.info.permissions.clone()))
    }

    /// Checks if the current skill is disabled
    ///
    /// # Returns
    /// True if the skill is disabled, false otherwise
    #[rhai_fn(return_raw)]
    pub fn is_disabled(ctx: NativeCallContext) -> Result<bool, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(skill_context.info.disabled)
    }
}
