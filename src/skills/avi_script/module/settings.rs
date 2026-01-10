use crate::config::{Setting, SettingNamed};
use crate::skills::avi_script::helpers::{
    skill_context, skill_context_def, yaml_to_dynamic,
};
use rhai::plugin::*;
use rhai::{Dynamic, NativeCallContext};
use std::collections::HashMap;

#[export_module]
pub mod settings_module {
    /// Gets a setting value for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the setting
    ///
    /// # Returns
    /// The setting value, or default if not set
    pub fn get(ctx: NativeCallContext, name: String) -> Option<Dynamic> {
        skill_context(ctx, None, |v| {
            Some(yaml_to_dynamic(&v.config.setting(&name)?.value))
        })
    }

    /// Lists all settings available for the current skill
    ///
    /// # Returns
    /// A list of setting names
    pub fn list(ctx: NativeCallContext) -> HashMap<String, Setting> {
        skill_context_def(ctx, |v| v.config.list_settings())
    }

    /// Checks if a setting exists for the current skill
    ///
    /// # Arguments
    /// * `name` - The name of the setting
    ///
    /// # Returns
    /// True if the setting exists, false otherwise
    pub fn has(ctx: NativeCallContext, name: String) -> bool {
        skill_context_def(ctx, |v| v.config.has_setting(&name))
    }

    /// Gets the full setting object including metadata
    ///
    /// # Arguments
    /// * `name` - The name of the setting
    ///
    /// # Returns
    /// The full setting object, or UNIT if not found
    pub fn full(ctx: NativeCallContext, name: String) -> Option<SettingNamed> {
        skill_context(ctx, None, |v| v.config.get_setting_full(&name))
    }
}
