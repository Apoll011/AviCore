use crate::skills::dsl::std::TINVOTS;
use dyon::Type::{Array, Bool, Object, Str};
use dyon::{Dfn, Module, Runtime, Variable};
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("object");

    module.add_str("has", has, Dfn::nl(vec![Object, Str], Bool));
    module.add_str("keys", keys, Dfn::nl(vec![Object], Array(Box::new(Str))));
}

pub(crate) fn has(rt: &mut Runtime) -> Result<Variable, String> {
    let key = rt.stack.pop().expect(TINVOTS);
    let key = match rt.get(&key) {
        &Variable::Str(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str")),
    };
    let obj = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::bool(match rt.get(&obj) {
        &Variable::Object(ref obj) => obj.contains_key(&key),
        x => return Err(rt.expected_arg(0, x, "object")),
    }))
}

pub(crate) fn keys(rt: &mut Runtime) -> Result<Variable, String> {
    let obj = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Array(Arc::new(match rt.get(&obj) {
        &Variable::Object(ref obj) => obj.keys().map(|k| Variable::Str(k.clone())).collect(),
        x => return Err(rt.expected_arg(0, x, "object")),
    })))
}
