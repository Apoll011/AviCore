use std::collections::HashMap;
use dyon::{dyon_fn, Runtime, Variable};
use crate::intent::{Intent, IntentInfo, JsonValue, Slot, SlotRange, SlotValue};
use dyon::embed::{PopVariable, PushVariable};
use std::sync::Arc;
use serde_json::Value;
/*fn main() {
    use dyon::{error, Runtime};

    let mut dyon_runtime = Runtime::new();
    let dyon_module = load_module().unwrap();

    if error(load("source/functions/loader.dyon", &mut module)) {
        None
    } else {
        Some(module)
    }

    if error(dyon_runtime.run(&Arc::new(dyon_module))) {
        return;
    }
}*/

pub fn load_module() -> Option<dyon::Module> {
    use dyon::Type::*;
    use dyon::{Dfn, Module};

    let mut module = Module::new();
    module.add_str("say_hello", say_hello, Dfn::nl(vec![], Void));

    Some(module)
}

dyon_fn! {fn say_hello() {
    println!("hi!");
}}

dyon_obj! {Intent { input, intent, slots}}
dyon_obj! { IntentInfo { intent_name, probability } }
dyon_obj! {Slot { raw_value, value, entity, slot_name, range }}
dyon_obj! {SlotValue { kind, value, grain, precision }}
dyon_obj! {SlotRange { start, end }}
impl PopVariable for JsonValue {
    fn pop_var(_rt: &Runtime, var: &Variable) -> Result<Self, String> {
        from_dyon_variable(var.clone())
    }
}

fn from_dyon_variable(var: Variable) -> Result<JsonValue, String> {
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
                values.push(from_dyon_variable(v.clone())?.0);
            }
            Ok(JsonValue(Value::Array(values)))
        }
        Object(o) => {
            let mut map = serde_json::Map::new();
            for (k, v) in o.iter() {
                map.insert(k.clone().to_string(), from_dyon_variable(v.clone())?.0);
            }
            Ok(JsonValue(Value::Object(map)))
        }
        Option(opt) => {
            match opt {
                Some(v) => from_dyon_variable(*v.clone()),
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
        to_dyon_variable(self.clone())
    }
}

fn to_dyon_variable(value: JsonValue) -> Variable {
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
            let arr: Vec<Variable> = vec.into_iter().map(|arg0: Value| to_dyon_variable(JsonValue(arg0))).collect();
            Array(Arc::new(arr))
        }
        JsonValue(Value::Object(map)) => {
            let mut obj: HashMap<Arc<String>, Variable> = HashMap::new();

            for (k, v) in map {
                obj.insert(Arc::new(k), to_dyon_variable(JsonValue(v)));
            }

            Object(Arc::new(obj))
        }
        _ => Bool(false, None),
    }
}