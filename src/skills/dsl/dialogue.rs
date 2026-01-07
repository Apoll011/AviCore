use crate::ctx::runtime;
use crate::dialogue::reply::RequestReply;
use crate::dialogue::response::{
    AnyValidator, BoolValidator, ListOrNoneValidator, MappedValidator, OptionalValidator,
    ResponseValidator,
};
use crate::dialogue::utils::{listen as device_listen, speak};
use crate::skills::dsl::avi_dsl::ctx;
use crate::skills::dsl::dyon_helpers::{dyon_obj_into_hashmap, hashmap_value_to_string};
use crate::{get_ctx, rt_spawn, speak, user_name};
use dyon::Type::*;
use dyon::embed::{PopVariable, PushVariable};
use dyon::{Dfn, Module, Runtime, Variable};
use std::collections::HashMap;
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("dialogue");
    module.add(Arc::new("say".into()), say, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("say_once".into()), say, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("listen".into()), listen, Dfn::nl(vec![], Void)); // Last device that sent a utterance will start listening again

    module.add(
        Arc::new("on_reply_any".into()),
        on_reply_any,
        Dfn::nl(vec![Str, Any], Void),
    );
    module.add(
        Arc::new("on_reply_list_or_none".into()),
        on_reply_list_or_none,
        Dfn::nl(vec![Str, Any], Void),
    );
    module.add(
        Arc::new("on_reply_optional".into()),
        on_reply_optional,
        Dfn::nl(vec![Str, Any], Void),
    );
    module.add(
        Arc::new("on_reply_bool".into()),
        on_reply_bool,
        Dfn::nl(vec![Str, Any], Void),
    );
    module.add(
        Arc::new("on_reply_mapped".into()),
        on_reply_mapped,
        Dfn::nl(vec![Str, Any], Void),
    );

    module.add(
        Arc::new("any_validator".into()),
        any_validator,
        Dfn::nl(vec![], Any),
    );
    module.add(
        Arc::new("list_or_none_validator".into()),
        list_or_none_validator,
        Dfn::nl(vec![Any], Any),
    );
    module.add(
        Arc::new("optional_validator".into()),
        optional_validator,
        Dfn::nl(vec![], Any),
    );
    module.add(
        Arc::new("bool_validator".into()),
        bool_validator,
        Dfn::nl(vec![Bool], Any),
    );
    module.add(
        Arc::new("mapped_validator".into()),
        mapped_validator,
        Dfn::nl(vec![Any, Any], Any),
    );

    module.add(
        Arc::new("confirm".into()),
        confirm,
        Dfn::nl(vec![Str, Str], Void),
    );
    module.add(Arc::new("repeat".into()), repeat, Dfn::nl(vec![], Void)); //Repeats the last spoken utterance (Dont matter the skill)

    module.add(
        Arc::new("request_attention".into()),
        req_attention,
        Dfn::nl(vec![], Void),
    ); //Call the user name without leaving the current skill
}

dyon_fn! {fn say(text: String) {
    speak(&text, true);
}}

dyon_fn! {fn req_attention() {
    speak!(&format!("{}!", user_name!()));
    device_listen();
}}

dyon_fn! {fn say_once(text: String) {
    speak!(&text);
}}

dyon_fn! {fn listen() {
    device_listen();
}}

dyon_fn! {fn repeat() {
    if let Some(v) = get_ctx!("utterance.last") { speak!(&v.to_string()) };
}}

dyon_obj! {AnyValidator { }}
dyon_obj! {ListOrNoneValidator { allowed_values }}
dyon_obj! {OptionalValidator { }}
dyon_obj! {BoolValidator { hard_search }}

impl<T: PopVariable + PushVariable + Clone> PopVariable for MappedValidator<T> {
    fn pop_var(rt: &Runtime, var: &Variable) -> Result<Self, String> {
        if let Variable::Object(obj) = var {
            let mut mappings = HashMap::new();
            let mut default = None;
            let mut hard_search = false;
            for (k, v) in obj.iter() {
                match &***k {
                    "default" => default = Some(T::pop_var(rt, v)?),
                    "hard_search" => {
                        if let Variable::Bool(b, _) = v {
                            hard_search = *b;
                        }
                    }
                    _ => {
                        mappings.insert((**k).clone(), T::pop_var(rt, v)?);
                    }
                }
            }
            let mut validator = MappedValidator::new(mappings);
            validator.default = default;
            validator.hard_search = hard_search;
            Ok(validator)
        } else {
            Err("Expected object for MappedValidator".to_string())
        }
    }
}

impl<T: PushVariable + Clone> PushVariable for MappedValidator<T> {
    fn push_var(&self) -> Variable {
        let mut map = HashMap::new();
        for (k, v) in &self.mappings {
            map.insert(Arc::new(k.clone()), PushVariable::push_var(v));
        }
        if let Some(ref def) = self.default {
            map.insert(Arc::new("default".to_string()), PushVariable::push_var(def));
        }
        map.insert(
            Arc::new("hard_search".to_string()),
            PushVariable::push_var(&self.hard_search),
        );

        Variable::Object(Arc::new(map))
    }
}

dyon_fn! {fn any_validator() -> AnyValidator {
    AnyValidator
}}

dyon_fn! {fn list_or_none_validator(allowed: Vec<String>) -> ListOrNoneValidator {
    ListOrNoneValidator::new(allowed)
}}

dyon_fn! {fn optional_validator() -> OptionalValidator {
    OptionalValidator
}}

dyon_fn! {fn bool_validator(hard_search: bool) -> BoolValidator {
    BoolValidator::new(hard_search)
}}

pub fn mapped_validator(_rt: &mut Runtime) -> Result<Variable, String> {
    let default: String = _rt.pop()?;
    let mappings = _rt.stack.pop();

    let mut validator =
        MappedValidator::new(hashmap_value_to_string(dyon_obj_into_hashmap(mappings)?));
    validator = validator.with_default(default);

    Ok(PushVariable::push_var(&validator))
}

fn handle_on_reply<V>(handler: String, validator: V, skill_name: String)
where
    V: ResponseValidator + Send + Sync + 'static,
    V::Output: std::fmt::Debug,
{
    rt_spawn! {
        if let Ok(c) = runtime() { c.reply_manager
        .set_reply(RequestReply {
            skill_request: skill_name,
            handler,
            validator: Box::new(validator),
        })
        .await };
    }
}

#[allow(non_snake_case)]
pub fn on_reply_any(rt: &mut Runtime) -> Result<(), String> {
    let validator: AnyValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_list_or_none(rt: &mut Runtime) -> Result<(), String> {
    let validator: ListOrNoneValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_optional(rt: &mut Runtime) -> Result<(), String> {
    let validator: OptionalValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_bool(rt: &mut Runtime) -> Result<(), String> {
    let validator: BoolValidator = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn on_reply_mapped(rt: &mut Runtime) -> Result<(), String> {
    let validator: MappedValidator<String> = rt.pop()?;
    let handler: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    handle_on_reply(handler, validator, skill_name);
    Ok(())
}

#[allow(non_snake_case)]
pub fn confirm(_rt: &mut Runtime) -> Result<(), String> {
    let handler: String = _rt.pop()?;
    let question_locale_id: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;
    let skill_name = ctx(_rt)?.info.id.clone();

    speak!(
        &skill_context
            .languages
            .get_translation(&question_locale_id)
            .unwrap()
    );

    handle_on_reply(handler, BoolValidator::new(false), skill_name);

    Ok(())
}
