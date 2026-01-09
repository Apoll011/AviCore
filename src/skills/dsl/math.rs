use crate::skills::dsl::std::TINVOTS;
use dyon::Type::{Array, Bool, F64};
use dyon::{Dfn, Module, Runtime, Variable};

pub fn add_functions(module: &mut Module) {
    module.ns("math");

    module.add_str("sqrt", sqrt, Dfn::nl(vec![F64], F64));
    module.add_str("sin", sin, Dfn::nl(vec![F64], F64));
    module.add_str("asin", asin, Dfn::nl(vec![F64], F64));
    module.add_str("cos", cos, Dfn::nl(vec![F64], F64));
    module.add_str("acos", acos, Dfn::nl(vec![F64], F64));
    module.add_str("tan", tan, Dfn::nl(vec![F64], F64));
    module.add_str("atan", atan, Dfn::nl(vec![F64], F64));
    module.add_str("atan2", atan2, Dfn::nl(vec![F64; 2], F64));
    module.add_str("exp", exp, Dfn::nl(vec![F64], F64));
    module.add_str("ln", ln, Dfn::nl(vec![F64], F64));
    module.add_str("log2", log2, Dfn::nl(vec![F64], F64));
    module.add_str("log10", log10, Dfn::nl(vec![F64], F64));
    module.add_str("round", round, Dfn::nl(vec![F64], F64));
    module.add_str("abs", abs, Dfn::nl(vec![F64], F64));
    module.add_str("floor", floor, Dfn::nl(vec![F64], F64));
    module.add_str("ceil", ceil, Dfn::nl(vec![F64], F64));
    #[cfg(not(target_family = "wasm"))]
    module.add_str("random", random, Dfn::nl(vec![], F64));
    module.add_str("tau", tau, Dfn::nl(vec![], F64));
    module.add_str("is_nan", is_nan, Dfn::nl(vec![F64], Bool));
    module.add_str("min", min, Dfn::nl(vec![Array(Box::new(F64))], F64));
    module.add_str("max", max, Dfn::nl(vec![Array(Box::new(F64))], F64));
}

dyon_fn! {fn sqrt(a: f64) -> f64 {a.sqrt()}}
dyon_fn! {fn sin(a: f64) -> f64 {a.sin()}}
dyon_fn! {fn asin(a: f64) -> f64 {a.asin()}}
dyon_fn! {fn cos(a: f64) -> f64 {a.cos()}}
dyon_fn! {fn acos(a: f64) -> f64 {a.acos()}}
dyon_fn! {fn tan(a: f64) -> f64 {a.tan()}}
dyon_fn! {fn atan(a: f64) -> f64 {a.atan()}}
dyon_fn! {fn atan2(y: f64, x: f64) -> f64 {y.atan2(x)}}
dyon_fn! {fn exp(a: f64) -> f64 {a.exp()}}
dyon_fn! {fn ln(a: f64) -> f64 {a.ln()}}
dyon_fn! {fn log2(a: f64) -> f64 {a.log2()}}
dyon_fn! {fn log10(a: f64) -> f64 {a.log10()}}
dyon_fn! {fn round(a: f64) -> f64 {a.round()}}
dyon_fn! {fn abs(a: f64) -> f64 {a.abs()}}
dyon_fn! {fn floor(a: f64) -> f64 {a.floor()}}
dyon_fn! {fn ceil(a: f64) -> f64 {a.ceil()}}

pub(crate) fn min(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::f64(match rt.get(&v) {
        &Variable::Array(ref arr) => {
            let mut min: f64 = ::std::f64::NAN;
            for v in &**arr {
                if let Variable::F64(val, _) = *rt.get(v) {
                    if val < min || min.is_nan() {
                        min = val
                    }
                }
            }
            min
        }
        x => return Err(rt.expected_arg(0, x, "array")),
    }))
}

pub(crate) fn max(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::f64(match rt.get(&v) {
        &Variable::Array(ref arr) => {
            let mut max: f64 = ::std::f64::NAN;
            for v in &**arr {
                if let Variable::F64(val, _) = *rt.get(v) {
                    if val > max || max.is_nan() {
                        max = val
                    }
                }
            }
            max
        }
        x => return Err(rt.expected_arg(0, x, "array")),
    }))
}

dyon_fn! {fn is_nan(v: f64) -> bool {v.is_nan()}}

pub(crate) fn random(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(Variable::f64(rand::random()))
}

dyon_fn! {fn tau() -> f64 {6.283_185_307_179_586}}
