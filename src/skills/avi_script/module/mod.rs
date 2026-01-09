use rhai::Engine;
use rhai::module_resolvers::StaticModuleResolver;

mod constant;
mod context;
mod dialogue;
mod json;
mod settings;

pub fn add(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();

    json::add(&mut resolver);
    constant::add(&mut resolver);
    settings::add(&mut resolver);
    dialogue::add(&mut resolver);
    context::add(&mut resolver);

    engine.set_module_resolver(resolver);
}
