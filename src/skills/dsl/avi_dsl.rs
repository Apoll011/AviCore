use std::collections::HashMap;
use dyon::{Runtime, Variable};
use crate::intent::{Intent, IntentInfo, JsonValue, Slot, SlotRange, SlotValue, YamlValue};
use dyon::embed::{PopVariable, PushVariable};
use std::sync::Arc;
use serde_json::Value;
use serde_yaml::{Value as Yaml, Mapping, Sequence};
use crate::ctx::RUNTIMECTX;
use crate::skills::skill_context::{ConstantNamed, IndividualLocale, Language, Manifest, Setting, SettingNamed, SkillContext};
use sha2::{Sha256, Digest};
use hmac::{Hmac, KeyInit, Mac};
use chrono::{Utc, DateTime};
use humantime::parse_duration;

pub fn load_module() -> Option<dyon::Module> {
    use dyon::Type::*;
    use dyon::{Dfn, Module};

    let mut module = Module::new();
    module.add_str("get_constant", get_constant, Dfn::nl(vec![Str], Any));
    module.add_str("get_setting", get_setting, Dfn::nl(vec![Str], Any));
    module.add_str("locale", locale, Dfn::nl(vec![Str], Any));

    module.add_str("get_setting_full", get_setting_full, Dfn::nl(vec![Str], Any));
    module.add_str("validate_setting", validate_setting, Dfn::nl(vec![Str], Any));
    module.add_str("list_settings", list_settings, Dfn::nl(vec![], Any));

    module.add_str("list_constants", list_constants, Dfn::nl(vec![], Any));
    module.add_str("has_constant", has_constant, Dfn::nl(vec![Str], Any));

    module.add_str("list_locales", list_locales, Dfn::nl(vec![Str], Any));

    module.add_str("get_manifest", get_manifest, Dfn::nl(vec![], Any));
    module.add_str("get_permissions", get_permissions, Dfn::nl(vec![], Any));
    module.add_str("is_disabled", is_disabled, Dfn::nl(vec![], Any));

    module.add_str("has_setting", has_setting, Dfn::nl(vec![Str], Any));

    module.add_str("json_parse", json_parse, Dfn::nl(vec![Str], Any));
    module.add_str("json_stringify", json_stringify, Dfn::nl(vec![Any], Str));

    module.add_str("crypto_hash", crypto_hash, Dfn::nl(vec![Str, Str], Str));
    module.add_str("crypto_hmac", crypto_hmac, Dfn::nl(vec![Str, Str, Str], Str));

    module.add_str("time_parse_duration", time_parse_duration, Dfn::nl(vec![Str], F64));
    module.add_str("time_format_date", time_format_date, Dfn::nl(vec![F64, Str], Str));


    Some(module)
}

fn ctx(rt: &mut Runtime) -> SkillContext {
    SkillContext::pop_var(rt, &rt.stack[0]).unwrap()
}

#[allow(non_snake_case)]
pub fn get_constant(_rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = _rt.pop()?;
    let skill_context = ctx(_rt);

    match skill_context.constant(&*name) {
        Some(v) => {
            Ok(PushVariable::push_var(v))
        }
        None => {
            Ok(PushVariable::push_var(&YamlValue(Yaml::Null)))
        }
    }
}

#[allow(non_snake_case)]
pub fn get_setting(_rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = _rt.pop()?;
    let skill_context = ctx(_rt);

    match skill_context.setting(&*name) {
        Some(v) => {
            Ok(PushVariable::push_var(&v.value.clone()))
        }
        None => {
            Ok(PushVariable::push_var(&YamlValue(Yaml::Null)))
        }
    }
}

#[allow(non_snake_case)]
pub fn locale(_rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt);

    match RUNTIMECTX.get() {
        Some(v) => {
            Ok(PushVariable::push_var(&skill_context.locale(&*v.lang, &*id).unwrap()))
        }
        None => {
            Ok(PushVariable::push_var(&YamlValue(Yaml::Null)))
        }
    }
}

#[allow(non_snake_case)]
pub fn list_constants(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);

    let list: Vec<(String, YamlValue)> =
        ctx.constants.iter()
            .map(|c| (c.name.clone(), c.value.clone()))
            .collect();

    Ok(PushVariable::push_var(&list))
}

#[allow(non_snakeCase)]
pub fn get_setting_full(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt);

    let val = ctx.settings.iter()
        .find(|s| s.name == name)
        .cloned()
        .unwrap_or(SettingNamed {
            name,
            setting: Setting::default()
        });

    Ok(PushVariable::push_var(&val))
}

#[allow(non_snake_case)]
pub fn validate_setting(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt);

    let Some(s) = ctx.settings.iter().find(|s| s.name == name) else {
        return Ok(PushVariable::push_var(&false));
    };

    let Setting { min, max, value, .. } = &s.setting;

    if let YamlValue(Yaml::Number(n)) = value {
        if let Some(min) = min {
            if n.as_i64().unwrap_or(0) < *min as i64 {
                return Ok(PushVariable::push_var(&false));
            }
        }
        if let Some(max) = max {
            if n.as_i64().unwrap_or(0) > *max as i64 {
                return Ok(PushVariable::push_var(&false));
            }
        }
        return Ok(PushVariable::push_var(&true));
    }

    Ok(PushVariable::push_var(&false))
}

#[allow(non_snake_case)]
pub fn list_settings(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);

    let list: Vec<(String, YamlValue)> =
        ctx.settings.iter()
            .map(|s| (s.name.clone(), s.setting.value.clone()))
            .collect();

    Ok(PushVariable::push_var(&list))
}

#[allow(non_snake_case)]
pub fn list_locales(rt: &mut Runtime) -> Result<Variable, String> {
    let code: String = rt.pop()?;
    let ctx = ctx(rt);

    let Some(lang) = ctx.languages.iter().find(|l| l.code == code) else {
        return Ok(PushVariable::push_var(&Vec::<(String, YamlValue)>::new()));
    };

    let pairs: Vec<(String, YamlValue)> = lang.lang
        .iter()
        .map(|i| (i.id.clone(), i.value.clone()))
        .collect();

    Ok(PushVariable::push_var(&pairs))
}

#[allow(non_snake_case)]
pub fn get_manifest(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);
    Ok(PushVariable::push_var(&ctx.info))
}

#[allow(non_snake_case)]
pub fn get_permissions(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);
    Ok(PushVariable::push_var(&ctx.info.permissions))
}

#[allow(non_snake_case)]
pub fn is_disabled(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);
    Ok(PushVariable::push_var(&ctx.info.disabled))
}

#[allow(non_snake_case)]
pub fn has_constant(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt);

    let exists = ctx.constants.iter().any(|c| c.name == name);
    Ok(PushVariable::push_var(&exists))
}

#[allow(non_snake_case)]
pub fn has_setting(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt);

    let exists = ctx.settings.iter().any(|s| s.name == name);
    Ok(PushVariable::push_var(&exists))
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

type HmacSha256 = Hmac<Sha256>;

#[allow(non_snake_case)]
pub fn crypto_hash(rt: &mut Runtime) -> Result<Variable, String> {
    let algo: String = rt.pop()?; // always "sha256"
    let data: String = rt.pop()?;

    if algo != "sha256" {
        return Err("Unsupported hash algorithm".to_string());
    }

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let out = format!("{:?}", hasher.finalize());

    Ok(PushVariable::push_var(&out))
}

#[allow(non_snake_case)]
pub fn crypto_hmac(rt: &mut Runtime) -> Result<Variable, String> {
    let algo: String = rt.pop()?;   // "sha256"
    let message: String = rt.pop()?;
    let key: String = rt.pop()?;

    if algo != "sha256" {
        return Err("Unsupported HMAC algorithm".to_string());
    }

    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|_| "Invalid HMAC key".to_string())?;

    mac.update(message.as_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();

    Ok(PushVariable::push_var(&hex::encode(bytes)))
}

#[allow(non_snake_case)]
pub fn time_parse_duration(rt: &mut Runtime) -> Result<Variable, String> {
    let s: String = rt.pop()?;
    match parse_duration(&s) {
        Ok(d) => Ok(PushVariable::push_var(&(d.as_millis() as f64))),
        Err(e) => Err(format!("Duration parse error: {}", e)),
    }
}

#[allow(non_snake_case)]
pub fn time_format_date(rt: &mut Runtime) -> Result<Variable, String> {
    let fmt: String = rt.pop()?;
    let millis: f64 = rt.pop()?; // input from DSL: time object = millis
    let dt = DateTime::<Utc>::from(std::time::UNIX_EPOCH + std::time::Duration::from_millis(millis as u64));

    match dt.format(&fmt).to_string().as_str() {
        s => Ok(PushVariable::push_var(&(s.parse::<String>().unwrap()))),
    }
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
dyon_obj! {Manifest { id, name, description, disabled, entry, capabilities, permissions, author, version }}
dyon_obj! {SkillContext { info, settings, constants, languages }}
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