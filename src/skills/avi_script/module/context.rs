use crate::skills::avi_script::helpers::{get_skill_context, json_to_dynamic};
use crate::{get_ctx, has_ctx, remove_ctx, set_ctx};
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, NativeCallContext, Position};

#[export_module]
pub mod context_module {
    /// Gets a value from the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key of the value to retrieve
    ///
    /// # Returns
    /// The value associated with the key, or UNIT if not found
    #[rhai_fn(return_raw)]
    pub fn get(ctx: NativeCallContext, key: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(get_ctx!(skill: skill_context.info.name, &key)
            .map(|v| json_to_dynamic(v))
            .unwrap_or(Dynamic::UNIT))
    }

    /// Checks if a key exists in the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// True if the key exists, false otherwise
    #[rhai_fn(return_raw)]
    pub fn has(ctx: NativeCallContext, key: String) -> Result<bool, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(has_ctx!(skill: skill_context.info.name, &key))
    }

    /// Removes a value from the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key of the value to remove
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(return_raw)]
    pub fn remove(ctx: NativeCallContext, key: String) -> Result<(), Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        let _ = remove_ctx!(skill: skill_context.info.name, &key);
        Ok(())
    }

    /// Sets a value in the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to store
    /// * `ttl` - Time to live in seconds (0 for no TTL)
    /// * `persist` - Whether to persist the value across sessions
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(return_raw)]
    pub fn set(
        ctx: NativeCallContext,
        key: String,
        value: Dynamic,
        ttl: u64,
        persist: bool,
    ) -> Result<(), Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        set_ctx!(skill: skill_context.info.name, key, value, ttl, persist);
        Ok(())
    }
}
