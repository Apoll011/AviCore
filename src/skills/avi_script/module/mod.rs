use rhai::Engine;
use rhai::module_resolvers::StaticModuleResolver;

mod constant;
mod context;
mod dialogue;
mod fs;
mod json;
mod locale;
mod log;
mod settings;
mod skill;
mod slots;

pub fn add(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();

    json::add(&mut resolver);
    constant::add(&mut resolver);
    settings::add(&mut resolver);
    dialogue::add(&mut resolver);
    context::add(&mut resolver);
    fs::add(&mut resolver);
    slots::add(&mut resolver);
    log::add(&mut resolver);
    locale::add(&mut resolver);
    skill::add(&mut resolver);

    engine.set_module_resolver(resolver);
}
