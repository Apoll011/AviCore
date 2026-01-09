use crate::skills::avi_script::helpers::{dynamic_to_json, json_to_dynamic};
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, Position};
use serde_json::Value;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("parse").set_into_module(&mut module, parse_json);
    FuncRegistration::new("to_json").set_into_module(&mut module, to_json);

    resolver.insert("json", module);
}

fn parse_json(json_str: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    match serde_json::from_str::<Value>(json_str) {
        Ok(value) => Ok(json_to_dynamic(value)),
        Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
            format!("JSON parse error: {}", err).into(),
            Position::NONE,
        ))),
    }
}

fn to_json(value: Dynamic) -> Result<String, Box<EvalAltResult>> {
    match dynamic_to_json(value) {
        Ok(json_value) => match serde_json::to_string_pretty(&json_value) {
            Ok(json_str) => Ok(json_str),
            Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("JSON stringify error: {}", err).into(),
                Position::NONE,
            ))),
        },
        Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
            err.to_string().parse().unwrap(),
            Position::NONE,
        ))),
    }
}
