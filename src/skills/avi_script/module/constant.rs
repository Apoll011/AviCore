use crate::dialogue::intent::YamlValue;
use crate::register_skill_func;
use crate::skills::avi_script::helpers::yaml_to_dynamic;
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Module};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    register_skill_func!(&mut module, "get", (name: String), |skill_context| {
        yaml_to_dynamic(&skill_context.config.constant(&name).unwrap_or(YamlValue(serde_yaml::Value::Null)))
    });
    register_skill_func!(&mut module, "list", (), |skill_context| {
        skill_context.config.list_constants()
    });
    register_skill_func!(&mut module, "has", (name: String), |skill_context| {
        skill_context.config.has_constant(&name)
    });

    resolver.insert("constant", module);
}
