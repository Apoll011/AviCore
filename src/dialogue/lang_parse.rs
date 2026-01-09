use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Position;
use rhai::TypeBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, CustomType, Clone)]
#[allow(dead_code)]
pub struct ExtractNumbers(pub Vec<f64>);

#[derive(Debug, Deserialize, CustomType, Clone)]
#[allow(dead_code)]
pub struct ExtractNumber(pub Option<f64>);

#[derive(Debug, Deserialize, CustomType, Clone)]
#[allow(dead_code)]
pub struct ExtractDuration {
    pub duration: Option<f64>,
    pub leftover: String,
}

#[derive(Debug, Deserialize, CustomType, Clone)]
#[allow(dead_code)]
pub struct ExtractDatetime {
    pub datetime: Option<String>,
    pub leftover: String,
}

#[derive(Debug, Deserialize, CustomType, Clone)]
#[allow(dead_code)]
pub struct IsFractional(pub Option<f64>);
