use rhai::NativeCallContext;
use rhai::plugin::*;

#[export_module]
pub mod skill_module {
    use crate::skills::avi_script::helpers::skill_context_def;
    use crate::skills::skill_context::Manifest;

    /// Gets the root directory of the current skill
    ///
    /// # Returns
    /// The path to the skill's root directory
    pub fn dir(ctx: NativeCallContext) -> String {
        skill_context_def(ctx, |v| v.path.clone())
    }

    /// Gets the version of the current skill
    ///
    /// # Returns
    /// The version string of the skill
    pub fn version(ctx: NativeCallContext) -> String {
        skill_context_def(ctx, |v| v.info.version.clone())
    }

    /// Gets the manifest information of the current skill
    ///
    /// # Returns
    /// A map containing the skill's manifest
    pub fn manifest(ctx: NativeCallContext) -> Manifest {
        skill_context_def(ctx, |v| v.info.clone())
    }

    /// Gets the permissions required by the current skill
    ///
    /// # Returns
    /// A list of permissions
    pub fn get_permissions(ctx: NativeCallContext) -> Vec<String> {
        skill_context_def(ctx, |v| v.info.permissions.clone())
    }

    /// Checks if the current skill is disabled
    ///
    /// # Returns
    /// True if the skill is disabled, false otherwise
    pub fn is_disabled(ctx: NativeCallContext) -> bool {
        skill_context_def(ctx, |v| v.info.disabled.clone())
    }
}
