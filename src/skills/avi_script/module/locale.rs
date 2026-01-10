use crate::dialogue::languages::lang;
use crate::skills::avi_script::helpers::get_skill_context;
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, Map, NativeCallContext, Position};

#[export_module]
pub mod locale_module {
    /// Gets a translation for a given ID in the current locale
    ///
    /// # Arguments
    /// * `id` - The ID of the translation to retrieve
    ///
    /// # Returns
    /// The translation string if found, or UNIT if not found
    #[rhai_fn(return_raw)]
    pub fn get(ctx: NativeCallContext, id: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;

        match skill_context.languages.get_translation(id) {
            Some(translation) => Ok(Dynamic::from(translation)),
            None => Ok(Dynamic::UNIT),
        }
    }

    /// Gets a formatted translation for a given ID in the current locale
    ///
    /// # Arguments
    /// * `id` - The ID of the translation to retrieve
    /// * `params` - A map of parameters to interpolate into the translation
    ///
    /// # Returns
    /// The formatted translation string if found, or UNIT if not found
    #[rhai_fn(return_raw)]
    pub fn get_fmt(
        ctx: NativeCallContext,
        id: &str,
        params: Map,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        let current_lang = lang();

        // Convert Map to HashMap<String, String>
        let string_params = map_to_string_hashmap(params)?;

        match skill_context
            .languages
            .locale_fmt(&current_lang, id, &string_params)
        {
            Some(formatted) => Ok(Dynamic::from(formatted)),
            None => Ok(Dynamic::UNIT),
        }
    }

    /// Lists all translations for a given locale code
    ///
    /// # Arguments
    /// * `code` - The locale code (e.g., 'en-US')
    ///
    /// # Returns
    /// A map of translations
    #[rhai_fn(return_raw)]
    pub fn list(ctx: NativeCallContext, code: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(Dynamic::from(skill_context.languages.list(code)))
    }

    /// Checks if a translation exists for a given ID
    ///
    /// # Arguments
    /// * `id` - The ID of the translation to check
    ///
    /// # Returns
    /// True if the translation exists, false otherwise
    #[rhai_fn(return_raw)]
    pub fn has(ctx: NativeCallContext, id: &str) -> Result<bool, Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        Ok(skill_context.languages.has(id))
    }

    /// Gets the current language code
    ///
    /// # Returns
    /// The current language code (e.g., 'en-US')

    pub fn current(_ctx: NativeCallContext) -> String {
        lang()
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
