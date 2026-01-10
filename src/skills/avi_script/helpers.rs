use crate::skills::skill_context::SkillContext;
use rhai::{Dynamic, Map, NativeCallContext, Variant};
use serde_json::Value;
use std::result::Result;

#[macro_export]
macro_rules! register_skill_func {
    ($module:expr, $name:expr, ($($param:ident : $type:ty),*), $default:expr, $comments:expr, $params_info:expr, |$skill_ctx:ident| $body:expr) => {
        rhai::FuncRegistration::new($name)
            .with_namespace(rhai::FnNamespace::Global)
            .with_comments($comments)
            .with_params_info($params_info)
            .set_into_module($module, |ctx: rhai::NativeCallContext, $($param : $type),*| {
                $crate::skills::avi_script::helpers::skill_context(ctx, $default, |$skill_ctx| $body)
            });
    };
    ($module:expr, $name:expr, ($($param:ident : $type:ty),*), $comments:expr, $params_info:expr, |$skill_ctx:ident| $body:expr) => {
        register_skill_func!($module, $name, ($($param : $type),*), ::std::default::Default::default(), $comments, $params_info, |$skill_ctx| $body);
    };
    ($module:expr, $name:expr, ($($param:ident : $type:ty),*), $default:expr, |$skill_ctx:ident| $body:expr) => {
        register_skill_func!($module, $name, ($($param : $type),*), $default, &[] as &[&str], &[] as &[&str], |$skill_ctx| $body);
    };
    ($module:expr, $name:expr, ($($param:ident : $type:ty),*), |$skill_ctx:ident| $body:expr) => {
        register_skill_func!($module, $name, ($($param : $type),*), ::std::default::Default::default(), &[] as &[&str], &[] as &[&str], |$skill_ctx| $body);
    };
}

pub fn json_to_dynamic(value: Value) -> Dynamic {
    match value {
        Value::Null => Dynamic::UNIT,
        Value::Bool(b) => Dynamic::from(b),
        Value::Number(n) => {
            if n.is_i64() {
                Dynamic::from(n.as_i64().unwrap())
            } else if n.is_u64() {
                Dynamic::from(n.as_u64().unwrap() as i64)
            } else {
                Dynamic::from(n.as_f64().unwrap())
            }
        }
        Value::String(s) => Dynamic::from(s),
        Value::Array(arr) => {
            let mut rhai_array = Vec::new();
            for item in arr {
                rhai_array.push(json_to_dynamic(item));
            }
            Dynamic::from(rhai_array)
        }
        Value::Object(obj) => {
            let mut rhai_map = Map::new();
            for (key, value) in obj {
                rhai_map.insert(key.into(), json_to_dynamic(value));
            }
            Dynamic::from(rhai_map)
        }
    }
}

pub fn dynamic_to_json(value: Dynamic) -> Result<Value, String> {
    match value.type_name() {
        "()" => Ok(Value::Null),
        "bool" => Ok(Value::Bool(value.as_bool().unwrap())),
        "i32" | "i64" => Ok(Value::Number(serde_json::Number::from(
            value.as_int().unwrap(),
        ))),
        "f32" | "f64" => {
            let float_val = value.as_float().unwrap();
            match serde_json::Number::from_f64(float_val) {
                Some(num) => Ok(Value::Number(num)),
                None => Err(format!("Cannot convert float {} to JSON number", float_val)),
            }
        }
        "string" | "ImmutableString" => {
            Ok(Value::String(value.into_immutable_string()?.to_string()))
        }
        "array" => {
            let array = value.into_array().unwrap();
            let mut json_array = Vec::new();
            for item in array {
                json_array.push(dynamic_to_json(item)?);
            }
            Ok(Value::Array(json_array))
        }
        "map" => {
            let map: Map = value.cast::<Map>();
            let mut json_map = serde_json::Map::new();
            for (key, val) in map {
                json_map.insert(key.to_string(), dynamic_to_json(val)?);
            }
            Ok(Value::Object(json_map))
        }
        other => Err(format!("Cannot convert {} to JSON", other)),
    }
}

pub fn yaml_to_dynamic(val: &serde_yaml::Value) -> Dynamic {
    match val {
        serde_yaml::Value::Null => Dynamic::UNIT,
        serde_yaml::Value::Bool(b) => Dynamic::from(*b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Dynamic::from(i)
            } else if let Some(f) = n.as_f64() {
                Dynamic::from(f)
            } else {
                Dynamic::UNIT
            }
        }
        serde_yaml::Value::String(s) => Dynamic::from(s.clone()),
        serde_yaml::Value::Sequence(seq) => {
            Dynamic::from_array(seq.iter().map(|v| yaml_to_dynamic(&v.clone())).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let mut dmap = Map::new();
            for (k, v) in map {
                if let serde_yaml::Value::String(key) = k {
                    dmap.insert(key.clone().into(), yaml_to_dynamic(&v.clone()));
                }
            }
            Dynamic::from(dmap)
        }
        _ => Dynamic::UNIT,
    }
}

pub fn skill_context<T: Variant + Clone, F>(ctx: NativeCallContext, default: T, func: F) -> T
where
    F: Fn(SkillContext) -> T,
{
    let skill_context = match get_skill_context(&ctx) {
        Ok(skill_context) => skill_context,
        Err(_) => return default,
    };

    func(skill_context)
}

pub fn skill_context_def<T: Variant + Clone + Default, F>(ctx: NativeCallContext, func: F) -> T
where
    F: Fn(SkillContext) -> T,
{
    let skill_context = match get_skill_context(&ctx) {
        Ok(skill_context) => skill_context,
        Err(_) => return Default::default(),
    };

    func(skill_context)
}

pub fn get_skill_name(ctx: &NativeCallContext) -> Result<String, String> {
    Ok(get_skill_context(ctx)?.info.name)
}

pub fn get_skill_context(ctx: &NativeCallContext) -> Result<SkillContext, String> {
    let tag = match ctx.tag() {
        Some(t) => t,
        None => return Err("Error getting tag".to_string()),
    };

    let skill_context = match tag.clone().try_cast::<SkillContext>() {
        Some(c) => c,
        None => return Err("Error casting skill context".to_string()),
    };

    Ok(skill_context)
}
