use crate::skills::avi_script::helpers::get_skill_context;
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, NativeCallContext};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("dir")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the root directory of the current skill",
            "/// ",
            "/// # Returns",
            "/// The path to the skill's root directory",
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, dir);

    FuncRegistration::new("version")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the version of the current skill",
            "/// ",
            "/// # Returns",
            "/// The version string of the skill",
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, version);

    FuncRegistration::new("manifest")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the manifest information of the current skill",
            "/// ",
            "/// # Returns",
            "/// A map containing the skill's manifest",
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, get_manifest);

    FuncRegistration::new("get_permissions")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the permissions required by the current skill",
            "/// ",
            "/// # Returns",
            "/// A list of permissions",
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, get_permissions);

    FuncRegistration::new("is_disabled")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Checks if the current skill is disabled",
            "/// ",
            "/// # Returns",
            "/// True if the skill is disabled, false otherwise",
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, is_disabled);

    resolver.insert("skill", module);
}

fn dir(ctx: NativeCallContext) -> Result<String, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(skill_context.path.clone())
}

fn version(ctx: NativeCallContext) -> Result<String, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(skill_context.info.version.clone())
}

fn get_manifest(ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(Dynamic::from(skill_context.info.clone()))
}

fn get_permissions(ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(Dynamic::from(skill_context.info.permissions.clone()))
}

fn is_disabled(ctx: NativeCallContext) -> Result<bool, Box<EvalAltResult>> {
    let skill_context = get_skill_context(&ctx)?;
    Ok(skill_context.info.disabled)
}
