use dyon::Variable;
use dyon::embed::PushVariable;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

pub fn dyon_obj_into_hashmap(obj: Option<Variable>) -> Result<HashMap<String, Value>, String> {
    match obj {
        Some(Variable::Object(v)) => {
            let mut map = HashMap::new();
            for (k, v) in v.iter() {
                let json_value = match v {
                    Variable::Str(text) => json!(text.as_ref()),
                    Variable::F64(number, ..) => json!(number),
                    Variable::Bool(bool_val, ..) => json!(bool_val),
                    Variable::Vec4(arr) => json!(arr),
                    Variable::Mat4(matrix) => json!(matrix.as_ref()),
                    Variable::Array(arr) => {
                        let array_values: Result<Vec<Value>, String> =
                            arr.iter().map(|item| variable_to_json(item)).collect();
                        json!(array_values?)
                    }
                    Variable::Object(inner_obj) => {
                        json!(dyon_obj_into_hashmap(Some(Variable::Object(
                            inner_obj.clone()
                        )))?)
                    }
                    Variable::Option(opt) => match opt {
                        Some(inner) => variable_to_json(inner)?,
                        None => Value::Null,
                    },
                    Variable::Result(res) => match res {
                        Ok(inner) => variable_to_json(inner)?,
                        Err(err) => json!({ "error": variable_to_json(&err.message) }),
                    },
                    _ => continue,
                };
                map.insert(k.as_ref().clone(), json_value);
            }
            Ok(map)
        }
        _ => Err(format!("Expected object, got {:?}", obj)),
    }
}

pub fn variable_to_json(var: &Variable) -> Result<Value, String> {
    match var {
        Variable::Str(text) => Ok(json!(text.as_ref())),
        Variable::F64(number, ..) => Ok(json!(number)),
        Variable::Bool(bool_val, ..) => Ok(json!(bool_val)),
        Variable::Vec4(arr) => Ok(json!(arr)),
        Variable::Mat4(matrix) => Ok(json!(matrix.as_ref())),
        Variable::Array(arr) => {
            let array_values: Result<Vec<Value>, String> =
                arr.iter().map(|item| variable_to_json(item)).collect();
            Ok(json!(array_values?))
        }
        Variable::Object(obj) => Ok(json!(dyon_obj_into_hashmap(Some(Variable::Object(
            obj.clone()
        )))?)),
        Variable::Option(opt) => match opt {
            Some(inner) => variable_to_json(inner),
            None => Ok(Value::Null),
        },
        Variable::Result(res) => match res {
            Ok(inner) => variable_to_json(inner),
            Err(err) => Ok(json!({ "error": format!("{:?}", err.message) })),
        },
        _ => Err(format!("Unsupported variable type: {:?}", var)),
    }
}

pub fn hashmap_value_to_string(map: HashMap<String, Value>) -> HashMap<String, String> {
    map.into_iter()
        .map(|(k, v)| (k, value_to_string(v)))
        .collect()
}

pub fn value_to_string(value: Value) -> String {
    match value {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            format!(
                "[{}]",
                arr.into_iter()
                    .map(value_to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Value::Object(obj) => {
            format!(
                "{{{}}}",
                obj.into_iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, value_to_string(v)))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

pub fn dyon_variable_to_json(value: &Value) -> Variable {
    use dyon::Variable::*;
    match value {
        Value::Bool(b) => PushVariable::push_var(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                PushVariable::push_var(&(i as f64))
            } else if let Some(f) = n.as_f64() {
                PushVariable::push_var(&f)
            } else {
                PushVariable::push_var(&n.as_f64().unwrap_or(0.0))
            }
        }
        Value::String(s) => PushVariable::push_var(s),
        Value::Array(vec) => {
            let arr: Vec<Variable> = vec
                .into_iter()
                .map(|arg0: &Value| dyon_variable_to_json(arg0))
                .collect();
            PushVariable::push_var(&arr)
        }
        Value::Object(map) => {
            let mut obj: HashMap<Arc<String>, Variable> = HashMap::new();

            for (k, v) in map {
                obj.insert(Arc::new(k.clone()), dyon_variable_to_json(v));
            }

            Object(Arc::new(obj))
        }
        _ => Option(None),
    }
}
