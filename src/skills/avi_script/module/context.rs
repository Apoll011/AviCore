use crate::skills::avi_script::helpers::json_to_dynamic;
use crate::{get_ctx, has_ctx, register_skill_func, remove_ctx, set_ctx};
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Dynamic, Module};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    register_skill_func!(&mut module, "get", (key: String), |skill_context| {
        get_ctx!(skill: skill_context.info.name, &key).map(|v| json_to_dynamic(v))
    });
    register_skill_func!(&mut module, "has", (key: String), |skill_context| {
        has_ctx!(skill: skill_context.info.name, &key)
    });
    register_skill_func!(&mut module, "remove", (key: String), |skill_context| {
        let _ = remove_ctx!(skill: skill_context.info.name, &key);
    });
    register_skill_func!(&mut module, "set", (key: String, value: Dynamic, ttl: u64, persist: bool), |skill_context| {
        set_ctx!(skill: skill_context.info.name, key, value, ttl, persist);
    });

    resolver.insert("context", module);
}
