use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, NativeCallContext, Map, Position};
use rhai::module_resolvers::StaticModuleResolver;
use crate::dialogue::languages::lang;
use crate::skills::avi_script::helpers::get_skill_context;
use crate::skills::skill_context::SkillContext;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("get")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets a translation for a given ID in the current locale",
            "/// ",
            "/// # Arguments",
            "/// * `id` - The ID of the translation to retrieve",
            "/// ",
            "/// # Returns",
            "/// The translation string if found, or UNIT if not found"
        ])
        .with_params_info(&["id: &str"])
        .set_into_module(&mut module, locale_get);

    FuncRegistration::new("get_fmt")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets a formatted translation for a given ID in the current locale",
            "/// ",
            "/// # Arguments",
            "/// * `id` - The ID of the translation to retrieve",
            "/// * `params` - A map of parameters to interpolate into the translation",
            "/// ",
            "/// # Returns",
            "/// The formatted translation string if found, or UNIT if not found"
        ])
        .with_params_info(&["id: &str", "params: Map"])
        .set_into_module(&mut module, locale_get_fmt);

    FuncRegistration::new("list")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Lists all translations for a given locale code",
            "/// ",
            "/// # Arguments",
            "/// * `code` - The locale code (e.g., 'en-US')",
            "/// ",
            "/// # Returns",
            "/// A map of translations"
        ])
        .with_params_info(&["code: &str"])
        .set_into_module(&mut module, list_locales);

    FuncRegistration::new("has")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Checks if a translation exists for a given ID",
            "/// ",
            "/// # Arguments",
            "/// * `id` - The ID of the translation to check",
            "/// ",
            "/// # Returns",
            "/// True if the translation exists, false otherwise"
        ])
        .with_params_info(&["id: &str"])
        .set_into_module(&mut module, has_locale);

    FuncRegistration::new("current")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the current language code",
            "/// ",
            "/// # Returns",
            "/// The current language code (e.g., 'en-US')"
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, current_lang);

    resolver.insert("locale", module);
}

fn locale_get(ctx: NativeCallContext, id: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;

    match skill_context.languages.get_translation(id) {
        Some(translation) => Ok(Dynamic::from(translation)),
        None => Ok(Dynamic::UNIT),
    }
}

fn locale_get_fmt(
    ctx: NativeCallContext,
    id: &str,
    params: Map,
) -> Result<Dynamic, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    let current_lang = lang();

    // Convert Map to HashMap<String, String>
    let string_params = map_to_string_hashmap(params)?;

    match skill_context.languages.locale_fmt(&current_lang, id, &string_params) {
        Some(formatted) => Ok(Dynamic::from(formatted)),
        None => Ok(Dynamic::UNIT),
    }
}

fn current_lang(_ctx: NativeCallContext) -> String {
    lang()
}

fn list_locales(ctx: NativeCallContext, code: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(Dynamic::from(skill_context.languages.list(code)))
}

fn has_locale(ctx: NativeCallContext, id: &str) -> Result<bool, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(skill_context.languages.has(id))
}

fn map_to_string_hashmap(map: Map) -> Result<std::collections::HashMap<String, String>, Box<EvalAltResult>> {
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
