use crate::skills::avi_script::helpers::{dynamic_to_json, json_to_dynamic};
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, Position};
use serde_json::Value;

#[export_module]
pub mod json_module {
    /// Parses a JSON string into a Rhai object
    ///
    /// # Arguments
    /// * `json_str` - The JSON string to parse
    ///
    /// # Returns
    /// A Rhai object representing the JSON data
    #[rhai_fn(global, return_raw)]
    pub fn parse(json_str: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        match serde_json::from_str::<Value>(json_str) {
            Ok(value) => Ok(json_to_dynamic(value)),
            Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("JSON parse error: {}", err).into(),
                Position::NONE,
            ))),
        }
    }

    /// Converts a Rhai object into a pretty-printed JSON string
    ///
    /// # Arguments
    /// * `value` - The Rhai object to convert
    ///
    /// # Returns
    /// A JSON string
    #[rhai_fn(global, return_raw)]
    pub fn to_json(value: Dynamic) -> Result<String, Box<EvalAltResult>> {
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
}
