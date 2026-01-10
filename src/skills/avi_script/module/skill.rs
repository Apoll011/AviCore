use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, NativeCallContext, Position};
use rhai::module_resolvers::StaticModuleResolver;
use crate::skills::avi_script::helpers::get_skill_context;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("dir")
        .with_namespace(rhai::FnNamespace::Global)
        .set_into_module(&mut module, dir);

    FuncRegistration::new("version")
        .with_namespace(rhai::FnNamespace::Global)
        .set_into_module(&mut module, version);

    FuncRegistration::new("manifest")
        .with_namespace(rhai::FnNamespace::Global)
        .set_into_module(&mut module, get_manifest);

    FuncRegistration::new("get_permissions")
        .with_namespace(rhai::FnNamespace::Global)
        .set_into_module(&mut module, get_permissions);

    FuncRegistration::new("is_disabled")
        .with_namespace(rhai::FnNamespace::Global)
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
