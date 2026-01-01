use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Variable};
use dyon::Type::*;
use crate::dialogue::response::{AnyValidator, BoolValidator, ListOrNoneValidator, MappedValidator, OptionalValidator};
use crate::dialogue::utils::{speak, listen as device_listen};

pub fn add_functions(module: &mut Module) {
    module.ns("dialogue");
    module.add(Arc::new("say".into()), say, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("listen".into()), listen, Dfn::nl(vec![Any], Any)); // Last device that sent a utterance will start listening again
    module.add(Arc::new("on_reply".into()), on_reply, Dfn::nl(vec![Any], Str)); //Sets a handles for the next user sopke text
    module.add(Arc::new("any_validator".into()), any_validator, Dfn::nl(vec![], Any));
    module.add(Arc::new("list_or_none_validator".into()), list_or_none_validator, Dfn::nl(vec![Any, Str], Any));
    module.add(Arc::new("optional_validator".into()), optional_validator, Dfn::nl(vec![Str], Any));
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
dyon_obj!{ListOrNoneValidator { allowed_values, none_text, case_sensitive }}
dyon_obj!{OptionalValidator { none_text }}
dyon_obj!{BoolValidator { yes_text, no_text, always_text, never_text, hard_search }}
dyon_obj!{MappedValidatorString { mappings, default, hard_search }}
dyon_obj!{MappedValidatorF64 { mappings, default, hard_search }}

pub type MappedValidatorString = MappedValidator<String>;
pub type MappedValidatorF64 = MappedValidator<f64>;

dyon_fn!{fn any_validator() -> AnyValidator {
        AnyValidator
    }}

dyon_fn!{fn list_or_none_validator(allowed: Vec<String>, none: String) -> ListOrNoneValidator {
        ListOrNoneValidator::new(allowed, none)
    }}

dyon_fn!{fn optional_validator(none: String) -> OptionalValidator {
        OptionalValidator::new(none)
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

dyon_fn!{fn on_reply(callback_name: String, _validator: Variable) {
        println!("on_reply registered:");
        println!("  Callback function: {}", callback_name);
        println!("  Validator: <validator object>");
        // TODO: Store callback_name and validator for when user input arrives
}}