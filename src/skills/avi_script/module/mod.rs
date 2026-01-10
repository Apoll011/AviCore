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
mod user;
mod util;

pub fn add(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();

    resolver.insert("log", rhai::exported_module!(log::log_module));
    resolver.insert(
        "dialogue",
        rhai::exported_module!(dialogue::dialogue_module),
    );
    resolver.insert("skill", rhai::exported_module!(skill::skill_module));
    resolver.insert("locale", rhai::exported_module!(locale::locale_module));
    resolver.insert("json", rhai::exported_module!(json::json_module));
    resolver.insert(
        "constant",
        rhai::exported_module!(constant::constant_module),
    );
    resolver.insert(
        "settings",
        rhai::exported_module!(settings::settings_module),
    );
    resolver.insert("context", rhai::exported_module!(context::context_module));
    resolver.insert("fs", rhai::exported_module!(fs::fs_module));
    resolver.insert("slots", rhai::exported_module!(slots::slots_module));
    resolver.insert("user", rhai::exported_module!(user::user_module));
    resolver.insert("util", rhai::exported_module!(util::util_module));
    engine.set_module_resolver(resolver);
}

pub(crate) fn add_static_modules(engine: &mut Engine) {
    engine.register_static_module("log", rhai::exported_module!(log::log_module).into());
    engine.register_static_module(
        "dialogue",
        rhai::exported_module!(dialogue::dialogue_module).into(),
    );
    engine.register_static_module("skill", rhai::exported_module!(skill::skill_module).into());
    engine.register_static_module(
        "locale",
        rhai::exported_module!(locale::locale_module).into(),
    );
    engine.register_static_module("json", rhai::exported_module!(json::json_module).into());
    engine.register_static_module(
        "constant",
        rhai::exported_module!(constant::constant_module).into(),
    );
    engine.register_static_module(
        "settings",
        rhai::exported_module!(settings::settings_module).into(),
    );
    engine.register_static_module(
        "context",
        rhai::exported_module!(context::context_module).into(),
    );
    engine.register_static_module("fs", rhai::exported_module!(fs::fs_module).into());
    engine.register_static_module("slots", rhai::exported_module!(slots::slots_module).into());
    engine.register_static_module("user", rhai::exported_module!(user::user_module).into());
    engine.register_static_module("util", rhai::exported_module!(util::util_module).into());
}
