use crate::config::{ConfigSystem, ConstantNamed, Setting, SettingNamed};
use crate::dialogue::intent::{
    Intent, IntentInfo, JsonValue, Slot, SlotRange, SlotValue, YamlValue,
};
use crate::dialogue::languages::{IndividualLocale, Language, LanguageSystem};
use crate::skills::skill_context::{Manifest, SkillContext};
use dyon::embed::{PopVariable, PushVariable};
use dyon::{Runtime, Variable};
use serde_json::Value;
use serde_yaml::{Mapping, Sequence, Value as Yaml};
use std::collections::HashMap;
use std::sync::Arc;

/// Loads and initializes the core Avi Dyon module with all sub-modules and functions.
pub fn load_module() -> Option<dyon::Module> {
    use dyon::Module;

    let mut module = Module::new();

    super::constants::add_functions(&mut module);
    super::settings::add_functions(&mut module);
    super::locales::add_functions(&mut module);
    super::manifest::add_functions(&mut module);
    super::utils::add_functions(&mut module);
    super::crypto::add_functions(&mut module);
    super::time::add_functions(&mut module);
    super::dialogue::add_functions(&mut module);
    super::intent::add_functions(&mut module);
    super::context::add_functions(&mut module);
    module.no_ns();

    Some(module)
}

/// Retrieves the `SkillContext` from the Dyon runtime stack.
///
/// # Arguments
///
/// * `rt` - The current Dyon runtime environment.
///
/// # Panics
///
/// Panics if the `SkillContext` is not found at the expected stack position or if it cannot be popped.
///
/// TODO: Avoid panicking; return an `Option` or `Result` instead.
pub fn ctx(rt: &mut Runtime) -> Result<SkillContext, String> {
    SkillContext::pop_var(rt, &rt.stack[0])
}

dyon_obj! {Intent { input, intent, slots}}
dyon_obj! { IntentInfo { intent_name, probability } }
dyon_obj! {Slot { raw_value, value, entity, slot_name, range }}
dyon_obj! {SlotValue { kind, value, grain, precision }}
dyon_obj! {SlotRange { start, end }}
dyon_obj! {SettingNamed { name, setting }}
dyon_obj! {ConstantNamed { name, value }}
dyon_obj! {IndividualLocale { id, value }}
dyon_obj! {Language { code, lang }}
dyon_obj! {Manifest { id, name, description, disabled, entry, capabilities, can_repeat_last_response, can_go_again, permissions, author, version }}
dyon_obj! {LanguageSystem { languages }}
dyon_obj! {ConfigSystem { settings, constants }}
dyon_obj! {SkillContext { path, info, config, languages }}
dyon_obj! {Setting {value, vtype, description, ui, required, min, max, enum_, advanced, group}}

impl PopVariable for JsonValue {
    /// Pops a `JsonValue` from the Dyon runtime.
    fn pop_var(_rt: &Runtime, var: &Variable) -> Result<Self, String> {
        from_dyon_variable_json(var.clone())
    }
}

/// Converts a Dyon `Variable` to a `JsonValue`.
///
/// # Errors
///
/// Returns an error if the Dyon variable is a complex type (Link/RustObject/UnsafeRef) that cannot be converted to JSON.
///
/// FIXME: `_ => todo!()` will cause a crash if an unhandled Dyon variable type is encountered.
fn from_dyon_variable_json(var: Variable) -> Result<JsonValue, String> {
    use dyon::Variable::*;
    match var {
        F64(n, ..) => {
            // Dyon’s F64 can be either integer-like or float.
            Ok(JsonValue(Value::Number(
                serde_json::Number::from_f64(n).ok_or_else(|| format!("Invalid f64: {}", n))?,
            )))
        }
        Bool(b, _) => Ok(JsonValue(Value::Bool(b))),
        Str(s) => Ok(JsonValue(Value::String(s.clone().to_string()))),
        Array(arr) => {
            let mut values = Vec::with_capacity(arr.len());
            for v in &*arr {
                values.push(from_dyon_variable_json(v.clone())?.0);
            }
            Ok(JsonValue(Value::Array(values)))
        }
        Object(o) => {
            let mut map = serde_json::Map::new();
            for (k, v) in o.iter() {
                map.insert(k.clone().to_string(), from_dyon_variable_json(v.clone())?.0);
            }
            Ok(JsonValue(Value::Object(map)))
        }
        Option(opt) => match opt {
            Some(v) => from_dyon_variable_json(*v.clone()),
            None => Ok(JsonValue(Value::Null)),
        },
        Link(_) | RustObject(_) | UnsafeRef(_) => {
            Err("Cannot convert complex Dyon types (Link/RustObject/UnsafeRef) to Value".into())
        }
        _ => todo!(),
    }
}

impl PushVariable for JsonValue {
    fn push_var(&self) -> Variable {
        to_dyon_variable_json(self.clone())
    }
}

fn to_dyon_variable_json(value: JsonValue) -> Variable {
    use dyon::Variable::*;
    match value {
        JsonValue(Value::Bool(b)) => Bool(b, None),
        JsonValue(Value::Number(n)) => {
            if let Some(i) = n.as_i64() {
                F64(i as f64, None)
            } else if let Some(f) = n.as_f64() {
                F64(f, None)
            } else {
                // fallback for very large numbers
                F64(n.as_f64().unwrap_or(0.0), None)
            }
        }
        JsonValue(Value::String(s)) => Str(Arc::new(s)),
        JsonValue(Value::Array(vec)) => {
            let arr: Vec<Variable> = vec
                .into_iter()
                .map(|arg0: Value| to_dyon_variable_json(JsonValue(arg0)))
                .collect();
            Array(Arc::new(arr))
        }
        JsonValue(Value::Object(map)) => {
            let mut obj: HashMap<Arc<String>, Variable> = HashMap::new();

            for (k, v) in map {
                obj.insert(Arc::new(k), to_dyon_variable_json(JsonValue(v)));
            }

            Object(Arc::new(obj))
        }
        _ => Bool(false, None),
    }
}

impl PopVariable for YamlValue {
    fn pop_var(_rt: &Runtime, var: &Variable) -> Result<Self, String> {
        from_dyon_variable(var.clone())
    }
}

fn from_dyon_variable(var: Variable) -> Result<YamlValue, String> {
    use dyon::Variable::*;

    match var {
        F64(n, ..) => {
            // YAML uses Number internally (i64, f64, etc.)
            if n.fract() == 0.0 {
                Ok(YamlValue(Yaml::Number((n as i64).into())))
            } else {
                Ok(YamlValue(Yaml::Number(n.into())))
            }
        }

        Bool(b, _) => Ok(YamlValue(Yaml::Bool(b))),

        Str(s) => Ok(YamlValue(Yaml::String((*s).clone()))),

        Array(arr) => {
            let mut seq = Sequence::with_capacity(arr.len());
            for v in arr.iter() {
                seq.push(from_dyon_variable(v.clone())?.0);
            }
            Ok(YamlValue(Yaml::Sequence(seq)))
        }

        Object(o) => {
            let mut map = Mapping::new();
            for (k, v) in o.iter() {
                let key = Yaml::String((**k).clone());
                let val = from_dyon_variable(v.clone())?.0;
                map.insert(key, val);
            }
            Ok(YamlValue(Yaml::Mapping(map)))
        }

        Option(opt) => match opt {
            Some(v) => from_dyon_variable(*v),
            None => Ok(YamlValue(Yaml::Null)),
        },

        Link(_) | RustObject(_) | UnsafeRef(_) => {
            Err("Cannot convert Dyon complex types (Link/RustObject/UnsafeRef) to YamlValue".into())
        }

        _ => Err("Unsupported Dyon type for YAML".into()),
    }
}

impl PushVariable for YamlValue {
    fn push_var(&self) -> Variable {
        to_dyon_variable(self.clone())
    }
}

fn to_dyon_variable(value: YamlValue) -> Variable {
    use dyon::Variable::*;
    match value.0 {
        Yaml::Bool(b) => Bool(b, None),

        Yaml::Number(n) => {
            if let Some(i) = n.as_i64() {
                F64(i as f64, None)
            } else if let Some(f) = n.as_f64() {
                F64(f, None)
            } else {
                // Fallback secure
                F64(0.0, None)
            }
        }

        Yaml::String(s) => Str(Arc::new(s)),

        Yaml::Sequence(seq) => {
            let arr: Vec<Variable> = seq
                .into_iter()
                .map(|v| to_dyon_variable(YamlValue(v)))
                .collect();
            Array(Arc::new(arr))
        }

        Yaml::Mapping(map) => {
            let mut obj: HashMap<Arc<String>, Variable> = HashMap::new();
            for (k, v) in map {
                let key = match k {
                    Yaml::String(s) => s,
                    _ => format!("{:?}", k),
                };
                obj.insert(Arc::new(key), to_dyon_variable(YamlValue(v)));
            }
            Object(Arc::new(obj))
        }

        Yaml::Null => Option(None),

        _ => Bool(false, None),
    }
}
