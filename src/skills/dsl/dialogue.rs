use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime};
use dyon::Type::*;
use crate::ctx::runtime;
use crate::dialogue::reply::RequestReply;
use crate::dialogue::response::{AnyValidator, BoolValidator, ListOrNoneValidator, MappedValidator, OptionalValidator, ResponseValidator};
use crate::dialogue::utils::{speak, listen as device_listen};
use crate::skills::dsl::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("dialogue");
    module.add(Arc::new("say".into()), say, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("listen".into()), listen, Dfn::nl(vec![Any], Void)); // Last device that sent a utterance will start listening again

    module.add(Arc::new("on_reply_any".into()), on_reply_any, Dfn::nl(vec![Str, Any], Void));
    module.add(Arc::new("on_reply_list_or_none".into()), on_reply_list_or_none, Dfn::nl(vec![Str, Any], Void));
    module.add(Arc::new("on_reply_optional".into()), on_reply_optional, Dfn::nl(vec![Str, Any], Void));
    module.add(Arc::new("on_reply_bool".into()), on_reply_bool, Dfn::nl(vec![Str, Any], Void));
    module.add(Arc::new("on_reply_mapped_str".into()), on_reply_mapped_str, Dfn::nl(vec![Str, Any], Void));
    module.add(Arc::new("on_reply_mapped_num".into()), on_reply_mapped_num, Dfn::nl(vec![Str, Any], Void));

    module.add(Arc::new("any_validator".into()), any_validator, Dfn::nl(vec![], Any));
    module.add(Arc::new("list_or_none_validator".into()), list_or_none_validator, Dfn::nl(vec![Any], Any));
    module.add(Arc::new("optional_validator".into()), optional_validator, Dfn::nl(vec![], Any));
    module.add(Arc::new("bool_validator".into()), bool_validator, Dfn::nl(vec![Str, Str, Str, Str], Any));
    module.add(Arc::new("mapped_validator_str".into()), mapped_validator_str, Dfn::nl(vec![Any, Any], Any));
    module.add(Arc::new("mapped_validator_num".into()), mapped_validator_num, Dfn::nl(vec![Any, Any], Any));
    /*module.add(Arc::new("ask".into()), dir, Dfn::nl(vec![], Str)); //Ask a question with a list of asnwers, fuzzy the response or frist second trird etc
    module.add(Arc::new("confirm".into()), dir, Dfn::nl(vec![], Str)); //Ask a yes or no question
    module.add(Arc::new("repeat".into()), dir, Dfn::nl(vec![], Str)); //Repeats the last spoken utterance (Dont matter the skill)
    module.add(Arc::new("request_attention".into()), dir, Dfn::nl(vec![], Str)); //Call the user name without leaving the current skill */
}

dyon_fn! {fn say(text: String) {
    speak(&text);
}}

dyon_fn! {fn listen() {
    device_listen();
}}

dyon_obj!{AnyValidator { }}
dyon_obj!{ListOrNoneValidator { allowed_values }}
dyon_obj!{OptionalValidator { }}
dyon_obj!{BoolValidator { yes_text, no_text, always_text, never_text, hard_search }}
dyon_obj!{MappedValidatorString { mappings, default, hard_search }}
dyon_obj!{MappedValidatorF64 { mappings, default, hard_search }}

pub type MappedValidatorString = MappedValidator<String>;
pub type MappedValidatorF64 = MappedValidator<f64>;

dyon_fn!{fn any_validator() -> AnyValidator {
        AnyValidator
    }}

dyon_fn!{fn list_or_none_validator(allowed: Vec<String>) -> ListOrNoneValidator {
        ListOrNoneValidator::new(allowed)
    }}

dyon_fn!{fn optional_validator() -> OptionalValidator {
        OptionalValidator
    }}

dyon_fn!{fn bool_validator(yes: String, no: String, always: String, never: String) -> BoolValidator {
        BoolValidator::new(yes, no, always, never)
    }}

dyon_fn!{fn mapped_validator_str(mappings: Vec<(String, String)>, default: std::option::Option<String>) -> MappedValidatorString {
        let mut validator = MappedValidator::new(mappings);
        if let Some(def) = default {
            validator = validator.with_default(def);
        }
        validator
    }}

dyon_fn!{fn mapped_validator_num(mappings: Vec<(String, f64)>, default: std::option::Option<f64>) -> MappedValidatorF64 {
        let mut validator = MappedValidator::new(mappings);
        if let Some(def) = default {
            validator = validator.with_default(def);
        }
        validator
}}

fn handle_on_reply<V>(
    handler: String,
    validator: V,
    skill_name: String
)
where
    V: ResponseValidator + Send + Sync + 'static,
    V::Output: std::fmt::Debug,
{
    runtime().rt.spawn(async move {
        runtime().reply_manager.set_reply(RequestReply {
            skill_request: skill_name,
            handler,
            validator: Box::new(validator),
        }).await;
    });
}

#[allow(non_snake_case)]
pub fn on_reply_any(rt: &mut Runtime) -> Result<(), String> {
    let validator: AnyValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_list_or_none(rt: &mut Runtime) -> Result<(), String> {
    let validator: ListOrNoneValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_optional(rt: &mut Runtime) -> Result<(), String> {
    let validator: OptionalValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_bool(rt: &mut Runtime) -> Result<(), String> {
    let validator: BoolValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_mapped_str(rt: &mut Runtime) -> Result<(), String> {
    let validator: MappedValidatorString = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_mapped_num(rt: &mut Runtime) -> Result<(), String> {
    let validator: MappedValidatorF64 = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}