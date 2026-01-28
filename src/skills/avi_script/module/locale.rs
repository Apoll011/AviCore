use crate::dialogue::languages::lang;
use crate::skills::avi_script::helpers::{skill_context, skill_context_def};
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, Map, NativeCallContext};

#[export_module]
pub mod locale_module {
    use std::collections::HashMap;

    /// Gets a translation for a given ID in the current locale
    ///
    /// # Arguments
    /// * `id` - The ID of the translation to retrieve
    ///
    /// # Returns
    /// The translation ImmutableString if found, or None if not found
    #[rhai_fn(return_raw)]
    pub fn get(
        ctx: NativeCallContext,
        id: ImmutableString,
    ) -> Result<ImmutableString, Box<EvalAltResult>> {
        skill_context(ctx, None, |v| v.languages.get_translation(&id))
            .ok_or(Box::new(EvalAltResult::ErrorRuntime(
                "Could not get the skill context".to_string().into(),
                Position::NONE,
            )))
            .map(ImmutableString::from)
    }

    /// Gets a formatted translation for a given ID in the current locale
    ///
    /// # Arguments
    /// * `id` - The ID of the translation to retrieve
    /// * `params` - A map of parameters to interpolate into the translation
    ///
    /// # Returns
    /// The formatted translation ImmutableString if found, or UNIT if not found
    #[rhai_fn(return_raw)]
    pub fn get_fmt(
        ctx: NativeCallContext,
        id: ImmutableString,
        params: Map,
    ) -> Result<ImmutableString, Box<EvalAltResult>> {
        let string_params = map_to_string_hashmap(params)?;
        skill_context(ctx, None, |v| {
            v.languages.locale_fmt(&lang(), &id, &string_params)
        })
        .ok_or(Box::new(EvalAltResult::ErrorRuntime(
            "Could not get the skill context".to_string().into(),
            Position::NONE,
        )))
        .map(ImmutableString::from)
    }

    /// Lists all translations for a given locale code
    ///
    /// # Arguments
    /// * `code` - The locale code (e.g., 'en-US')
    ///
    /// # Returns
    /// A map of translations
    pub fn list(
        ctx: NativeCallContext,
        code: ImmutableString,
    ) -> HashMap<ImmutableString, serde_yaml::Value> {
        skill_context_def(ctx, |v| v.languages.list(&code))
            .iter()
            .map(|(k, v)| (ImmutableString::from(k), v.clone()))
            .collect()
    }

    /// Checks if a translation exists for a given ID
    ///
    /// # Arguments
    /// * `id` - The ID of the translation to check
    ///
    /// # Returns
    /// True if the translation exists, false otherwise
    pub fn has(ctx: NativeCallContext, id: ImmutableString) -> bool {
        skill_context_def(ctx, |v| v.languages.has(&id))
    }

    /// Gets the current language code
    ///
    /// # Returns
    /// The current language code (e.g., 'en-US')
    pub fn current(_ctx: NativeCallContext) -> ImmutableString {
        ImmutableString::from(lang())
    }
}

fn map_to_string_hashmap(
    map: Map,
) -> Result<std::collections::HashMap<String, String>, Box<EvalAltResult>> {
    let mut result = std::collections::HashMap::new();

    for (key, value) in map {
        let string_value = dynamic_to_string(value)?;
        result.insert(key.to_string(), string_value);
    }

    Ok(result)
}

fn dynamic_to_string(value: Dynamic) -> Result<String, Box<EvalAltResult>> {
    if let Some(s) = value.clone().try_cast::<String>() {
        Ok(s)
    } else if let Some(i) = value.clone().try_cast::<i64>() {
        Ok(i.to_string())
    } else if let Some(f) = value.clone().try_cast::<f64>() {
        Ok(f.to_string())
    } else if let Some(b) = value.clone().try_cast::<bool>() {
        Ok(b.to_string())
    } else {
        Ok(format!("{:?}", value))
    }
}
