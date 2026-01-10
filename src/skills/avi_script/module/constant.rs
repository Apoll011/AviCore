use crate::skills::avi_script::helpers::{get_skill_context, yaml_to_dynamic};
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, NativeCallContext, Position};

#[export_module]
pub mod constant_module {
    /// Gets a constant value defined for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the constant
    ///
    /// # Returns
    /// The constant value, or UNIT if not found
    #[rhai_fn(return_raw)]
    pub fn get(ctx: NativeCallContext, name: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(yaml_to_dynamic(
            &skill_context
                .config
                .constant(&name)
                .unwrap_or(serde_yaml::Value::Null),
        ))
    }

    /// Lists all constants available for the current skill
    ///
    /// # Returns
    /// A list of constant names
    #[rhai_fn(return_raw)]
    pub fn list(ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(skill_context.config.list_constants()))
    }

    /// Checks if a constant exists for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the constant
    ///
    /// # Returns
    /// True if the constant exists, false otherwise
    #[rhai_fn(return_raw)]
    pub fn has(ctx: NativeCallContext, name: String) -> Result<bool, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(skill_context.config.has_constant(&name))
    }
}
