use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Variable};
use dyon::Type::*;
use crate::dialogue::response::{AnyValidator, BoolValidator, ListOrNoneValidator, MappedValidator, OptionalValidator, ResponseValidator};
use crate::dialogue::utils::{speak, listen as device_listen};

pub fn add_functions(module: &mut Module) {
    module.ns("dialogue");
    module.add(Arc::new("say".into()), say, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("listen".into()), listen, Dfn::nl(vec![Any], Void)); // Last device that sent a utterance will start listening again
    module.add(Arc::new("on_reply".into()), on_reply, Dfn::nl(vec![Str, Any], Void)); //Sets a handles for the next user sopke text
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

#[derive(Clone)]
pub enum AnyValidatorType {
    Any(AnyValidator),
    ListOrNone(ListOrNoneValidator),
    Optional(OptionalValidator),
    Bool(BoolValidator),
    MappedStr(MappedValidatorString),
    MappedNum(MappedValidatorF64),
}

 #[allow(non_snake_case)]
    pub fn on_reply(rt: &mut ::dyon::Runtime) -> Result<(), String> {
        fn inner(callback_name: String, validator: Variable) -> Result<(), String> {
            {
                println!("on_reply registered:");
                println!("  Callback function: {}", callback_name);

                let validator_type = match &validator {
                    Variable::RustObject(obj) => {
                        let guard = obj.lock().unwrap();
                        if let Some(v) = guard.downcast_ref::<AnyValidator>() {
                            println!("  Validator type: AnyValidator");
                            AnyValidatorType::Any(v.clone())
                        } else if let Some(v) = guard.downcast_ref::<ListOrNoneValidator>() {
                            println!("  Validator type: ListOrNoneValidator");
                            println!("    - Allowed values: {:?}", v.allowed_values);
                            println!("    - None text: {}", v.none_text);
                            AnyValidatorType::ListOrNone(v.clone())
                        } else if let Some(v) = guard.downcast_ref::<OptionalValidator>() {
                            println!("  Validator type: OptionalValidator");
                            println!("    - None text: {}", v.none_text);
                            AnyValidatorType::Optional(v.clone())
                        } else if let Some(v) = guard.downcast_ref::<BoolValidator>() {
                            println!("  Validator type: BoolValidator");
                            println!("    - Yes: {}, No: {}", v.yes_text, v.no_text);
                            println!("    - Always: {}, Never: {}", v.always_text, v.never_text);
                            AnyValidatorType::Bool(v.clone())
                        } else if let Some(v) = guard.downcast_ref::<MappedValidatorString>() {
                            println!("  Validator type: MappedValidator<String>");
                            println!("    - Mappings count: {}", v.mappings.len());
                            AnyValidatorType::MappedStr(v.clone())
                        } else if let Some(v) = guard.downcast_ref::<MappedValidatorF64>() {
                            println!("  Validator type: MappedValidator<f64>");
                            println!("    - Mappings count: {}", v.mappings.len());
                            AnyValidatorType::MappedNum(v.clone())
                        } else {
                            return Err("Unknown validator type".to_string());
                        }
                    }
                    _ => return Err("Validator must be a RustObject".to_string()),
                };

                Ok(())
            }
        }

        let validator: Variable = rt.pop()?;
        let callback_name: String = rt.pop()?;
        inner(callback_name, validator)?;
        Ok(())
    }