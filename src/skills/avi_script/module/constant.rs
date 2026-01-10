use crate::register_skill_func;
use crate::skills::avi_script::helpers::yaml_to_dynamic;
use rhai::Module;
use rhai::module_resolvers::StaticModuleResolver;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    register_skill_func!(
        &mut module,
        "get",
        (name: String),
        &[
            "/// Gets a constant value defined for the current skill",
            "/// ",
            "/// # Arguments",
            "/// * `name` - The name of the constant",
            "/// ",
            "/// # Returns",
            "/// The constant value, or UNIT if not found"
        ],
        &["name: String"],
        |skill_context| {
            yaml_to_dynamic(&skill_context.config.constant(&name).unwrap_or(serde_yaml::Value::Null))
        }
    );
    register_skill_func!(
        &mut module,
        "list",
        (),
        &[
            "/// Lists all constants available for the current skill",
            "/// ",
            "/// # Returns",
            "/// A list of constant names"
        ],
        &[] as &[&str],
        |skill_context| { skill_context.config.list_constants() }
    );
    register_skill_func!(
        &mut module,
        "has",
        (name: String),
        &[
            "/// Checks if a constant exists for the current skill",
            "/// ",
            "/// # Arguments",
            "/// * `name` - The name of the constant",
            "/// ",
            "/// # Returns",
            "/// True if the constant exists, false otherwise"
        ],
        &["name: String"],
        |skill_context| {
            skill_context.config.has_constant(&name)
        }
    );

    resolver.insert("constant", module);
}
