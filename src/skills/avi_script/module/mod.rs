use rhai::Engine;
use rhai::module_resolvers::StaticModuleResolver;

pub(crate) mod constant;
pub(crate) mod context;
pub(crate) mod dialogue;
pub(crate) mod fs;
pub(crate) mod json;
pub(crate) mod locale;
pub(crate) mod log;
pub(crate) mod settings;
pub(crate) mod skill;
pub(crate) mod slots;

pub fn add(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();

    resolver.insert("log", rhai::exported_module!(log::log_module));
    resolver.insert("dialogue", rhai::exported_module!(dialogue::dialogue_module));
    resolver.insert("skill", rhai::exported_module!(skill::skill_module));
    resolver.insert("locale", rhai::exported_module!(locale::locale_module));
    resolver.insert("json", rhai::exported_module!(json::json_module));
    resolver.insert("constant", rhai::exported_module!(constant::constant_module));
    resolver.insert("settings", rhai::exported_module!(settings::settings_module));
    resolver.insert("context", rhai::exported_module!(context::context_module));
    resolver.insert("fs", rhai::exported_module!(fs::fs_module));
    resolver.insert("slots", rhai::exported_module!(slots::slots_module));


    engine.set_module_resolver(resolver);
}