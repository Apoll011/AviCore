use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use super::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("manifest");
    module.add(Arc::new("get".into()), get_manifest, Dfn::nl(vec![], Any));
    module.add(Arc::new("get_permissions".into()), get_permissions, Dfn::nl(vec![], Any));
    module.add(Arc::new("is_disabled".into()), is_disabled, Dfn::nl(vec![], Any));
}

#[allow(non_snake_case)]
pub fn get_manifest(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);
    Ok(PushVariable::push_var(&ctx.info))
}

#[allow(non_snake_case)]
pub fn get_permissions(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);
    Ok(PushVariable::push_var(&ctx.info.permissions))
}

#[allow(non_snake_case)]
pub fn is_disabled(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);
    Ok(PushVariable::push_var(&ctx.info.disabled))
}
