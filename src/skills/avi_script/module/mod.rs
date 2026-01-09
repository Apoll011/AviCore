use rhai::Engine;
use rhai::module_resolvers::StaticModuleResolver;

mod json;

pub fn add(engine: &mut Engine) {
    let mut resolver = StaticModuleResolver::new();

    json::add(&mut resolver);

    engine.set_module_resolver(resolver);
}