use crate::register_skill_func;
use rhai::Module;
use rhai::module_resolvers::StaticModuleResolver;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    register_skill_func!(
        &mut module,
        "get",
        (name: String),
        &[
            "/// Gets a setting value for the current skill",
            "/// ",
            "/// # Arguments",
            "/// * `name` - The name of the setting",
            "/// ",
            "/// # Returns",
            "/// The setting value, or default if not set"
        ],
        &["name: String"],
        |skill_context| {
            skill_context.config.setting(&name).unwrap_or_default()
        }
    );
    register_skill_func!(
        &mut module,
        "list",
        (),
        &[
            "/// Lists all settings available for the current skill",
            "/// ",
            "/// # Returns",
            "/// A list of setting names"
        ],
        &[] as &[&str],
        |skill_context| { skill_context.config.list_settings() }
    );
    register_skill_func!(
        &mut module,
        "has",
        (name: String),
        &[
            "/// Checks if a setting exists for the current skill",
            "/// ",
            "/// # Arguments",
            "/// * `name` - The name of the setting",
            "/// ",
            "/// # Returns",
            "/// True if the setting exists, false otherwise"
        ],
        &["name: String"],
        |skill_context| {
            skill_context.config.has_setting(&name)
        }
    );
    register_skill_func!(
        &mut module,
        "full",
        (name: String),
        &[
            "/// Gets the full setting object including metadata",
            "/// ",
            "/// # Arguments",
            "/// * `name` - The name of the setting",
            "/// ",
            "/// # Returns",
            "/// The full setting object, or UNIT if not found"
        ],
        &["name: String"],
        |skill_context| {
            skill_context.config.get_setting_full(&name)
        }
    );

    resolver.insert("constant", module);
}
