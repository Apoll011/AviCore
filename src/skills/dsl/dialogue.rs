use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use crate::intent::{JsonValue};
use crate::skills::dsl::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("dialogue");
    module.add(Arc::new("say".into()), json_parse, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("listen".into()), json_parse, Dfn::nl(vec![Str], Any)); // Last device that sent a utterance will start listening again
    module.add(Arc::new("on_reply".into()), json_stringify, Dfn::nl(vec![Any], Str)); //Sets a handles for the next user sopke text
    module.add(Arc::new("ask".into()), dir, Dfn::nl(vec![], Str)); //Ask a question with a list of asnwers, fuzzy the response or frist second trird etc
    module.add(Arc::new("confirm".into()), dir, Dfn::nl(vec![], Str)); //Ask a yes or no question
    module.add(Arc::new("repeat".into()), dir, Dfn::nl(vec![], Str)); //Repeats the last spoken utterance (Dont matter the skill)
}

#[allow(non_snake_case)]
pub fn json_parse(rt: &mut Runtime) -> Result<Variable, String> {
    let s: String = rt.pop()?;
    match serde_json::from_str::<JsonValue>(&s) {
        Ok(v) => Ok(PushVariable::push_var(&v)),
        Err(e) => Err(format!("JSON parse error: {}", e)),
    }
}

#[allow(non_snake_case)]
pub fn json_stringify(rt: &mut Runtime) -> Result<Variable, String> {
    let obj: JsonValue = rt.pop()?;
    match serde_json::to_string(&obj) {
        Ok(s) => Ok(PushVariable::push_var(&s)),
        Err(e) => Err(format!("JSON stringify error: {}", e)),
    }
}

#[allow(non_snake_case)]
pub fn dir(_rt: &mut Runtime) -> Result<Variable, String> {
    let skill_context = ctx(_rt);
    Ok(PushVariable::push_var(&skill_context.path))
}