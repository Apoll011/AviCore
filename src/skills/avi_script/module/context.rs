use crate::skills::avi_script::helpers::json_to_dynamic;
use crate::skills::avi_script::helpers::skill_context_def;
use crate::{get_ctx, has_ctx, remove_ctx, set_ctx};
use rhai::plugin::*;
use rhai::{Dynamic, NativeCallContext};

#[export_module]
pub mod context_module {

    /// Gets a value from the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key of the value to retrieve
    ///
    /// # Returns
    /// The value associated with the key, or UNIT if not found
    #[rhai_fn(volatile)]
    pub fn get(ctx: NativeCallContext, key: String) -> Dynamic {
        skill_context_def(ctx, |v| {
            get_ctx!(skill: v.info.name.clone(), &key)
                .map(|v| json_to_dynamic(v))
                .unwrap_or(Dynamic::UNIT)
        })
    }

    /// Checks if a key exists in the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// True if the key exists, false otherwise
    #[rhai_fn(volatile)]
    pub fn has(ctx: NativeCallContext, key: String) -> bool {
        skill_context_def(ctx, |v| has_ctx!(skill: v.info.name.clone(), &key))
    }

    /// Removes a value from the skill's persistent context
    ///
    /// # Arguments
    /// * `key` - The key of the value to remove
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(volatile)]
    pub fn remove(ctx: NativeCallContext, key: String) {
        skill_context_def(ctx, |v| {
            let _ = remove_ctx!(skill: v.info.name.clone(), &key);
        });
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
    #[rhai_fn(volatile)]
    pub fn set(ctx: NativeCallContext, key: String, value: Dynamic, ttl: u64, persist: bool) {
        skill_context_def(
            ctx,
            |v| set_ctx!(skill: v.info.name.clone(), key, value, ttl, persist),
        );
    }
}
