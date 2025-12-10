use std::collections::HashMap;
use dyon::{dyon_fn, Runtime, Variable};
use crate::intent::{Intent, IntentInfo, JsonValue, Slot, SlotRange, SlotValue, YamlValue};
use dyon::embed::{PopVariable, PushVariable};
use std::sync::Arc;
use serde_json::Value;
use serde_yaml::{Value as Yaml, Mapping, Sequence};
use crate::skills::skill_context::{ConstantNamed, IndividualLocale, Language, Setting, SettingNamed, SkillContext};

pub fn load_module() -> Option<dyon::Module> {
    use dyon::Type::*;
    use dyon::{Dfn, Module};

    let mut module = Module::new();
    module.add_str("get_constant", get_constant, Dfn::nl(vec![Str, Any], Any));
    module.add_str("get_setting", get_setting, Dfn::nl(vec![Str, Any], Any));
    module.add_str("locale", locale, Dfn::nl(vec![Str, Any], Any));

    Some(module)
}

dyon_fn! {fn get_constant(name: String, skill_context: SkillContext) -> YamlValue {
    skill_context.constant(&*name).unwrap().clone()
}}

dyon_fn! {fn get_setting(name: String, skill_context: SkillContext) -> YamlValue {
    skill_context.setting(&*name).unwrap().clone().value
}}

dyon_fn! {fn locale(id: String, skill_context: SkillContext) -> YamlValue {
    skill_context.locale("pt", &*id).unwrap()
}}

dyon_obj! {Intent { input, intent, slots}}
dyon_obj! { IntentInfo { intent_name, probability } }
dyon_obj! {Slot { raw_value, value, entity, slot_name, range }}
dyon_obj! {SlotValue { kind, value, grain, precision }}
dyon_obj! {SlotRange { start, end }}
dyon_obj! {SettingNamed { name, setting }}
dyon_obj! {ConstantNamed { name, value }}
dyon_obj! {IndividualLocale { id, value }}
dyon_obj! {Language { code, lang }}
dyon_obj! {SkillContext { settings, constants, languages }}
dyon_obj! {Setting {value, vtype, description, ui, required, min, max, enum_, advanced, group}}
impl PopVariable for JsonValue {
    fn pop_var(_rt: &Runtime, var: &Variable) -> Result<Self, String> {
        from_dyon_variable_json(var.clone())
    }
}

fn from_dyon_variable_json(var: Variable) -> Result<JsonValue, String> {
    use dyon::Variable::*;
    match var {
        F64(n, ..) => {
            // Dyon’s F64 can be either integer-like or float.
            Ok(JsonValue(Value::Number(serde_json::Number::from_f64(n)
                .ok_or_else(|| format!("Invalid f64: {}", n))?)))
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
        Option(opt) => {
            match opt {
                Some(v) => from_dyon_variable_json(*v.clone()),
                None => Ok(JsonValue(Value::Null)),
            }
        }
        Link(_) | RustObject(_) | UnsafeRef(_) => {
            Err("Cannot convert complex Dyon types (Link/RustObject/UnsafeRef) to Value".into())
        },
        _ => todo!()
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
            let arr: Vec<Variable> = vec.into_iter().map(|arg0: Value| to_dyon_variable_json(JsonValue(arg0))).collect();
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

        Link(_) | RustObject(_) | UnsafeRef(_) =>
            Err("Cannot convert Dyon complex types (Link/RustObject/UnsafeRef) to YamlValue".into()),

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
                // Fallback seguro
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