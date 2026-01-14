use crate::skills::avi_script::helpers::skill_context_def;
use crate::skills::avi_script::helpers::yaml_to_dynamic;
use rhai::plugin::*;
use rhai::{Dynamic, NativeCallContext};
use std::collections::HashMap;

#[export_module]
pub mod constant_module {
    /// Gets a constant value defined for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the constant
    ///
    /// # Returns
    /// The constant value, or UNIT if not found
    #[rhai_fn(volatile)]
    pub fn get(ctx: NativeCallContext, name: ImmutableString) -> Dynamic {
        skill_context_def(ctx, |v| {
            yaml_to_dynamic(&v.config.constant(&name).unwrap_or(serde_yaml::Value::Null))
        })
    }

    /// Lists all constants available for the current skill
    ///
    /// # Returns
    /// A list of constant names
    #[rhai_fn(volatile)]
    pub fn list(ctx: NativeCallContext) -> HashMap<ImmutableString, Dynamic> {
        skill_context_def(ctx, |v| {
            v.config
                .list_constants()
                .iter()
                .map(|(k, v)| (ImmutableString::from(k), yaml_to_dynamic(v)))
                .collect()
        })
    }

    /// Checks if a constant exists for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the constant
    ///
    /// # Returns
    /// True if the constant exists, false otherwise
    #[rhai_fn(volatile)]
    pub fn has(ctx: NativeCallContext, name: ImmutableString) -> bool {
        skill_context_def(ctx, |v| v.config.has_constant(&name))
    }
}
