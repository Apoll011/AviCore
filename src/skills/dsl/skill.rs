use crate::skills::dsl::avi_dsl::ctx;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;

pub fn add_functions(module: &mut Module) {
    module.ns("skill");
    module.add_str("dir", dir, Dfn::nl(vec![], Str));
    module.add_str("version", version, Dfn::nl(vec![], Str));
    module.add_str("manifest", get_manifest, Dfn::nl(vec![], Any));
    module.add_str("get_permissions", get_permissions, Dfn::nl(vec![], Any));
    module.add_str("is_disabled", is_disabled, Dfn::nl(vec![], Any));
}

#[allow(non_snake_case)]
pub fn dir(_rt: &mut Runtime) -> Result<Variable, String> {
    let skill_context = ctx(_rt)?;
    Ok(PushVariable::push_var(&skill_context.path))
}

#[allow(non_snake_case)]
pub fn version(_rt: &mut Runtime) -> Result<Variable, String> {
    let skill_context = ctx(_rt)?;
    Ok(PushVariable::push_var(&skill_context.info.version))
}

#[allow(non_snake_case)]
pub fn get_manifest(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt)?;
    Ok(PushVariable::push_var(&ctx.info))
}

#[allow(non_snake_case)]
pub fn get_permissions(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt)?;
    Ok(PushVariable::push_var(&ctx.info.permissions))
}

#[allow(non_snake_case)]
pub fn is_disabled(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt)?;
    Ok(PushVariable::push_var(&ctx.info.disabled))
}
