use crate::skills::dsl::std::TINVOTS;
use dyon::Type::{Any, Option, Link, Bool};
use dyon::{Dfn, Module, Runtime, Variable};

pub fn add_functions(module: &mut Module) {
    module.ns("link");
    module.add_str("head", head, Dfn::nl(vec![Link], Any));
    module.add_str("tip", tip, Dfn::nl(vec![Link], Option(Box::new(Any))));
    module.add_str("tail", tail, Dfn::nl(vec![Link], Link));
    module.add_str("neck", neck, Dfn::nl(vec![Link], Link));
    module.add_str("is_empty", is_empty, Dfn::nl(vec![Link], Bool));
}

pub(crate) fn head(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Option(match rt.get(&v) {
        &Variable::Link(ref link) => link.head(),
        x => return Err(rt.expected_arg(0, x, "link")),
    }))
}

pub(crate) fn tip(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Option(match rt.get(&v) {
        &Variable::Link(ref link) => link.tip(),
        x => return Err(rt.expected_arg(0, x, "link")),
    }))
}

pub(crate) fn tail(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Link(Box::new(match rt.get(&v) {
        &Variable::Link(ref link) => link.tail(),
        x => return Err(rt.expected_arg(0, x, "link")),
    })))
}

pub(crate) fn neck(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Link(Box::new(match rt.get(&v) {
        &Variable::Link(ref link) => link.neck(),
        x => return Err(rt.expected_arg(0, x, "link")),
    })))
}

pub(crate) fn is_empty(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::bool(match rt.get(&v) {
        &Variable::Link(ref link) => link.is_empty(),
        x => return Err(rt.expected_arg(0, x, "link")),
    }))
}