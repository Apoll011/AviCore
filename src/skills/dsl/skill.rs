use crate::dialogue::intent::JsonValue;
use crate::skills::dsl::avi_dsl::ctx;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("skill");
    module.add(Arc::new("dir".into()), dir, Dfn::nl(vec![], Str));
}

#[allow(non_snake_case)]
pub fn dir(_rt: &mut Runtime) -> Result<Variable, String> {
    let skill_context = ctx(_rt)?;
    Ok(PushVariable::push_var(&skill_context.path))
}
