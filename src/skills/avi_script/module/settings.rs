use crate::register_skill_func;
use rhai::Module;
use rhai::module_resolvers::StaticModuleResolver;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    register_skill_func!(&mut module, "get", (name: String), |skill_context| {
        skill_context.config.setting(&name).unwrap_or_default()
    });
    register_skill_func!(&mut module, "list", (), |skill_context| {
        skill_context.config.list_settings()
    });
    register_skill_func!(&mut module, "has", (name: String), |skill_context| {
        skill_context.config.has_setting(&name)
    });
    register_skill_func!(&mut module, "full", (name: String), |skill_context| {
        skill_context.config.get_setting_full(&name)
    });

    resolver.insert("constant", module);
}
