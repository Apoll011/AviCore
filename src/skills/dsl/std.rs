#![allow(non_snake_case)]

use dyon::Type::{Any, Array, Bool, F64, Link, Secret, Str, Void};
use dyon::{
    Dfn, Error, LAZY_AND, LAZY_NO, LAZY_OR, LAZY_UNWRAP_OR, Lt, Mat4, Module, Runtime, Thread,
    Type, Variable, Vec4, load_str,
};
use lazy_static::lazy_static;
use std::f64::consts::PI;
use std::result::Result;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub(crate) static ref LESS: Arc<String> = Arc::new("less".into());
    pub(crate) static ref LESS_OR_EQUAL: Arc<String> = Arc::new("less_or_equal".into());
    pub(crate) static ref GREATER: Arc<String> = Arc::new("greater".into());
    pub(crate) static ref GREATER_OR_EQUAL: Arc<String> = Arc::new("greater_or_equal".into());
    pub(crate) static ref EQUAL: Arc<String> = Arc::new("equal".into());
    pub(crate) static ref NOT_EQUAL: Arc<String> = Arc::new("not_equal".into());
    pub(crate) static ref AND_ALSO: Arc<String> = Arc::new("and_also".into());
    pub(crate) static ref OR_ELSE: Arc<String> = Arc::new("or_else".into());
    pub(crate) static ref ADD: Arc<String> = Arc::new("add".into());
    pub(crate) static ref SUB: Arc<String> = Arc::new("sub".into());
    pub(crate) static ref MUL: Arc<String> = Arc::new("mul".into());
    pub(crate) static ref DIV: Arc<String> = Arc::new("div".into());
    pub(crate) static ref REM: Arc<String> = Arc::new("rem".into());
    pub(crate) static ref POW: Arc<String> = Arc::new("pow".into());
    pub(crate) static ref DOT: Arc<String> = Arc::new("dot".into());
    pub(crate) static ref CROSS: Arc<String> = Arc::new("cross".into());
    pub(crate) static ref NOT: Arc<String> = Arc::new("not".into());
    pub(crate) static ref NEG: Arc<String> = Arc::new("neg".into());
    pub(crate) static ref NORM: Arc<String> = Arc::new("norm".into());
    pub(crate) static ref T: Arc<String> = Arc::new("T".into());
}

lazy_static! {
    pub(crate) static ref TEXT_TYPE: Arc<String> = Arc::new("string".into());
    pub(crate) static ref F64_TYPE: Arc<String> = Arc::new("number".into());
    pub(crate) static ref VEC4_TYPE: Arc<String> = Arc::new("vec4".into());
    pub(crate) static ref MAT4_TYPE: Arc<String> = Arc::new("mat4".into());
    pub(crate) static ref RETURN_TYPE: Arc<String> = Arc::new("return".into());
    pub(crate) static ref BOOL_TYPE: Arc<String> = Arc::new("boolean".into());
    pub(crate) static ref OBJECT_TYPE: Arc<String> = Arc::new("object".into());
    pub(crate) static ref LINK_TYPE: Arc<String> = Arc::new("link".into());
    pub(crate) static ref ARRAY_TYPE: Arc<String> = Arc::new("array".into());
    pub(crate) static ref UNSAFE_REF_TYPE: Arc<String> = Arc::new("unsafe_ref".into());
    pub(crate) static ref REF_TYPE: Arc<String> = Arc::new("ref".into());
    pub(crate) static ref RUST_OBJECT_TYPE: Arc<String> = Arc::new("rust_object".into());
    pub(crate) static ref OPTION_TYPE: Arc<String> = Arc::new("option".into());
    pub(crate) static ref RESULT_TYPE: Arc<String> = Arc::new("result".into());
    pub(crate) static ref THREAD_TYPE: Arc<String> = Arc::new("thread".into());
    pub(crate) static ref CLOSURE_TYPE: Arc<String> = Arc::new("closure".into());
    pub(crate) static ref IN_TYPE: Arc<String> = Arc::new("in".into());
    pub(crate) static ref MAIN: Arc<String> = Arc::new("main".into());
}
pub const TINVOTS: &str = "There is no value on the stack";

pub(crate) fn and_also(rt: &mut Runtime) -> Result<Variable, String> {
    use dyon::Variable::*;

    let b = rt.stack.pop().expect(TINVOTS);
    let a = rt.stack.pop().expect(TINVOTS);
    Ok(match (rt.get(&a), rt.get(&b)) {
        (&Bool(a, ref sec), &Bool(b, _)) => Bool(a && b, sec.clone()),
        _ => return Err("Expected `bool`".into()),
    })
}

pub(crate) fn or_else(rt: &mut Runtime) -> Result<Variable, String> {
    use dyon::Variable::*;

    let b = rt.stack.pop().expect(TINVOTS);
    let a = rt.stack.pop().expect(TINVOTS);
    Ok(match (rt.get(&a), rt.get(&b)) {
        (&Bool(a, ref sec), &Bool(b, _)) => Bool(a || b, sec.clone()),
        _ => return Err("Expected `bool`".into()),
    })
}

pub(crate) fn less(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => Bool(a < b, sec.clone()),
        (&Str(ref a), &Str(ref b)) => Variable::bool(a < b),
        _ => return Err("Expected `f64` or `str`".into()),
    })
}

pub(crate) fn less_or_equal(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => Bool(a <= b, sec.clone()),
        (&Str(ref a), &Str(ref b)) => Variable::bool(a <= b),
        _ => return Err("Expected `f64` or `str`".into()),
    })
}

pub(crate) fn greater(a: &Variable, b: &Variable) -> Result<Variable, String> {
    less_or_equal(a, b).map(|v| {
        if let Variable::Bool(a, sec) = v {
            Variable::Bool(!a, sec)
        } else {
            panic!("Expected equal to return `bool`")
        }
    })
}

pub(crate) fn greater_or_equal(a: &Variable, b: &Variable) -> Result<Variable, String> {
    less(a, b).map(|v| {
        if let Variable::Bool(a, sec) = v {
            Variable::Bool(!a, sec)
        } else {
            panic!("Expected equal to return `bool`")
        }
    })
}

pub(crate) fn equal(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => Bool(a == b, sec.clone()),
        (&Str(ref a), &Str(ref b)) => Variable::bool(a == b),
        (&Bool(a, ref sec), &Bool(b, _)) => Bool(a == b, sec.clone()),
        (&Vec4(a), &Vec4(b)) => Variable::bool(a == b),
        (&Object(ref a), &Object(ref b)) => Variable::bool(
            a.len() == b.len()
                && a.iter().all(|a| {
                    if let Some(b_val) = b.get(a.0) {
                        matches!(equal(a.1, b_val), Ok(Bool(true, _)))
                    } else {
                        false
                    }
                }),
        ),
        (&Array(ref a), &Array(ref b)) => Variable::bool(
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(a, b)| matches!(equal(a, b), Ok(Bool(true, _)))),
        ),
        (&Option(None), &Option(None)) => Variable::bool(true),
        (&Option(None), &Option(_)) => Variable::bool(false),
        (&Option(_), &Option(None)) => Variable::bool(false),
        (&Option(Some(ref a)), &Option(Some(ref b))) => equal(a, b)?,
        _ => return Err("Expected `f64`, `str`, `bool`, `vec4`, `{}`, `[]` or `opt`".into()),
    })
}

pub(crate) fn not_equal(a: &Variable, b: &Variable) -> Result<Variable, String> {
    equal(a, b).map(|v| {
        if let Variable::Bool(a, sec) = v {
            Variable::Bool(!a, sec)
        } else {
            panic!("Expected equal to return `bool`")
        }
    })
}

pub(crate) fn add(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => F64(a + b, sec.clone()),
        (&Vec4(a), &Vec4(b)) => Vec4(vecmath::vec4_add(a, b)),
        (&Vec4(a), &F64(b, _)) | (&F64(b, _), &Vec4(a)) => {
            let b = b as f32;
            Vec4([a[0] + b, a[1] + b, a[2] + b, a[3] + b])
        }
        (&Mat4(ref a), &Mat4(ref b)) => Mat4(Box::new(vecmath::mat4_add(**a, **b))),
        (&F64(a, _), &Mat4(ref b)) | (&Mat4(ref b), &F64(a, _)) => {
            let a = a as f32;
            Mat4(Box::new([
                [b[0][0] + a, b[0][1] + a, b[0][2] + a, b[0][3] + a],
                [b[1][0] + a, b[1][1] + a, b[1][2] + a, b[1][3] + a],
                [b[2][0] + a, b[2][1] + a, b[2][2] + a, b[2][3] + a],
                [b[3][0] + a, b[3][1] + a, b[3][2] + a, b[3][3] + a],
            ]))
        }
        (&Bool(a, ref sec), &Bool(b, _)) => Bool(a || b, sec.clone()),
        (&Str(ref a), &Str(ref b)) => {
            let mut res = String::with_capacity(a.len() + b.len());
            res.push_str(a);
            res.push_str(b);
            Str(Arc::new(res))
        }
        (&Link(ref a), &Link(ref b)) => Link(Box::new(a.add(b))),
        _ => return Err("Expected `f64`, `vec4`, `mat4`, `bool`, `str` or `link`".into()),
    })
}

pub(crate) fn sub(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => F64(a - b, sec.clone()),
        (&Vec4(a), &Vec4(b)) => Vec4(vecmath::vec4_sub(a, b)),
        (&Vec4(a), &F64(b, _)) => {
            let b = b as f32;
            Vec4([a[0] - b, a[1] - b, a[2] - b, a[3] - b])
        }
        (&F64(a, _), &Vec4(b)) => {
            let a = a as f32;
            Vec4([a - b[0], a - b[1], a - b[2], a - b[3]])
        }
        (&Mat4(ref a), &Mat4(ref b)) => Mat4(Box::new(vecmath::mat4_sub(**a, **b))),
        (&F64(a, _), &Mat4(ref b)) => {
            let a = a as f32;
            Mat4(Box::new([
                [a - b[0][0], a - b[0][1], a - b[0][2], a - b[0][3]],
                [a - b[1][0], a - b[1][1], a - b[1][2], a - b[1][3]],
                [a - b[2][0], a - b[2][1], a - b[2][2], a - b[2][3]],
                [a - b[3][0], a - b[3][1], a - b[3][2], a - b[3][3]],
            ]))
        }
        (&Mat4(ref b), &F64(a, _)) => {
            let a = a as f32;
            Mat4(Box::new([
                [b[0][0] - a, b[0][1] - a, b[0][2] - a, b[0][3] - a],
                [b[1][0] - a, b[1][1] - a, b[1][2] - a, b[1][3] - a],
                [b[2][0] - a, b[2][1] - a, b[2][2] - a, b[2][3] - a],
                [b[3][0] - a, b[3][1] - a, b[3][2] - a, b[3][3] - a],
            ]))
        }
        (&Bool(a, ref sec), &Bool(b, _)) => Bool(a && !b, sec.clone()),
        _ => return Err("Expected `f64`, `vec4`, `mat4` or `bool`".into()),
    })
}

pub(crate) fn mul(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => F64(a * b, sec.clone()),
        (&Vec4(a), &Vec4(b)) => Vec4(vecmath::vec4_mul(a, b)),
        (&Vec4(a), &F64(b, _)) | (&F64(b, _), &Vec4(a)) => {
            let b = b as f32;
            Vec4([a[0] * b, a[1] * b, a[2] * b, a[3] * b])
        }
        (&Mat4(ref a), &Mat4(ref b)) => Mat4(Box::new(vecmath::col_mat4_mul(**a, **b))),
        (&F64(a, _), &Mat4(ref b)) | (&Mat4(ref b), &F64(a, _)) => {
            let a = a as f32;
            Mat4(Box::new([
                [b[0][0] * a, b[0][1] * a, b[0][2] * a, b[0][3] * a],
                [b[1][0] * a, b[1][1] * a, b[1][2] * a, b[1][3] * a],
                [b[2][0] * a, b[2][1] * a, b[2][2] * a, b[2][3] * a],
                [b[3][0] * a, b[3][1] * a, b[3][2] * a, b[3][3] * a],
            ]))
        }
        (&Mat4(ref a), &Vec4(b)) => Vec4(vecmath::col_mat4_transform(**a, b)),
        (&Bool(a, ref sec), &Bool(b, _)) => Bool(a && b, sec.clone()),
        _ => return Err("Expected `f64`, `vec4`, `mat4` or `bool`".into()),
    })
}

pub(crate) fn div(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => F64(a / b, sec.clone()),
        (&Vec4(a), &Vec4(b)) => Vec4([a[0] / b[0], a[1] / b[1], a[2] / b[2], a[3] / b[3]]),
        (&Vec4(a), &F64(b, _)) => {
            let b = b as f32;
            Vec4([a[0] / b, a[1] / b, a[2] / b, a[3] / b])
        }
        (&F64(a, _), &Vec4(b)) => {
            let a = a as f32;
            Vec4([a / b[0], a / b[1], a / b[2], a / b[3]])
        }
        _ => return Err("Expected `f64` or `vec4`".into()),
    })
}

pub(crate) fn rem(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => F64(a % b, sec.clone()),
        (&Vec4(a), &Vec4(b)) => Vec4([a[0] % b[0], a[1] % b[1], a[2] % b[2], a[3] % b[3]]),
        (&Vec4(a), &F64(b, _)) => {
            let b = b as f32;
            Vec4([a[0] % b, a[1] % b, a[2] % b, a[3] % b])
        }
        (&F64(a, _), &Vec4(b)) => {
            let a = a as f32;
            Vec4([a % b[0], a % b[1], a % b[2], a % b[3]])
        }
        _ => return Err("Expected `f64` or `vec4`".into()),
    })
}

pub(crate) fn pow(a: &Variable, b: &Variable) -> Result<Variable, String> {
    use dyon::Variable::*;

    Ok(match (a, b) {
        (&F64(a, ref sec), &F64(b, _)) => F64(a.powf(b), sec.clone()),
        (&Vec4(a), &Vec4(b)) => Vec4([
            a[0].powf(b[0]),
            a[1].powf(b[1]),
            a[2].powf(b[2]),
            a[3].powf(b[3]),
        ]),
        (&Vec4(a), &F64(b, _)) => {
            let b = b as f32;
            Vec4([a[0].powf(b), a[1].powf(b), a[2].powf(b), a[3].powf(b)])
        }
        (&F64(a, _), &Vec4(b)) => {
            let a = a as f32;
            Vec4([a.powf(b[0]), a.powf(b[1]), a.powf(b[2]), a.powf(b[3])])
        }
        (&Bool(a, ref sec), &Bool(ref b, _)) => Bool(a ^ b, sec.clone()),
        _ => return Err("Expected `f64`, `vec4` or `bool`".into()),
    })
}

pub(crate) fn not(a: &Variable) -> Result<Variable, String> {
    Ok(match *a {
        Variable::Bool(ref b, ref sec) => Variable::Bool(!b, sec.clone()),
        _ => return Err("Expected `bool`".into()),
    })
}

pub(crate) fn neg(a: &Variable) -> Result<Variable, String> {
    Ok(match *a {
        Variable::F64(v, ref sec) => Variable::F64(-v, sec.clone()),
        Variable::Vec4(v) => Variable::Vec4([-v[0], -v[1], -v[2], -v[3]]),
        Variable::Mat4(ref m) => Variable::Mat4(Box::new([
            [-m[0][0], -m[0][1], -m[0][2], -m[0][3]],
            [-m[1][0], -m[1][1], -m[1][2], -m[1][3]],
            [-m[2][0], -m[2][1], -m[2][2], -m[2][3]],
            [-m[3][0], -m[3][1], -m[3][2], -m[3][3]],
        ])),
        _ => return Err("Expected `f64`, `vec4` or `mat4`".into()),
    })
}

pub(crate) fn dot(a: &Variable, b: &Variable) -> Result<Variable, String> {
    Ok(Variable::f64(match (a, b) {
        (&Variable::Vec4(a), &Variable::Vec4(b)) => vecmath::vec4_dot(a, b) as f64,
        (&Variable::Vec4(a), &Variable::F64(b, _)) | (&Variable::F64(b, _), &Variable::Vec4(a)) => {
            let b = b as f32;
            (a[0] * b + a[1] * b + a[2] * b + a[3] * b) as f64
        }
        _ => return Err("Expected (vec4, vec4), (vec4, f64) or (f64, vec4)".into()),
    }))
}

fn deep_clone(var: &Variable, stack: &[Variable]) -> Variable {
    use Variable::*;

    match *var {
        F64(_, _) => var.clone(),
        Vec4(_) => var.clone(),
        Mat4(_) => var.clone(),
        Return => var.clone(),
        Bool(_, _) => var.clone(),
        Str(_) => var.clone(),
        Object(ref obj) => {
            let mut res = obj.clone();
            for val in Arc::make_mut(&mut res).values_mut() {
                *val = deep_clone(val, stack);
            }
            Object(res)
        }
        Array(ref arr) => {
            let mut res = arr.clone();
            for it in Arc::make_mut(&mut res) {
                *it = deep_clone(it, stack);
            }
            Array(res)
        }
        Link(_) => var.clone(),
        Ref(ind) => deep_clone(&stack[ind], stack),
        UnsafeRef(_) => panic!("Unsafe reference can not be cloned"),
        RustObject(_) => var.clone(),
        Option(None) => Option(None),
        Option(Some(ref v)) => Option(Some(v.clone())),
        Result(Ok(ref ok)) => Result(Ok(ok.clone())),
        Result(Err(ref err)) => Result(Err(err.clone())),
        Thread(_) => var.clone(),
        Closure(_, _) => var.clone(),
        In(_) => var.clone(),
    }
}
pub(crate) fn clone(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(deep_clone(rt.get(&v), &rt.stack))
}

pub(crate) fn why(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Array(Arc::new(match rt.get(&v) {
        &Variable::Bool(true, Some(ref sec)) => {
            let mut sec = (**sec).clone();
            sec.reverse();
            sec
        }
        &Variable::Bool(true, None) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "This does not make sense, perhaps an array is empty?".into()
            });
        }
        &Variable::Bool(false, _) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Must be `true` to have meaning, try add or remove `!`".into()
            });
        }
        x => return Err(rt.expected_arg(0, x, "bool")),
    })))
}

pub(crate) fn _where(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Array(Arc::new(match rt.get(&v) {
        &Variable::F64(val, Some(ref sec)) => {
            if val.is_nan() {
                return Err({
                    rt.arg_err_index.set(Some(0));
                    "Expected number, found `NaN`".into()
                });
            } else {
                let mut sec = (**sec).clone();
                sec.reverse();
                sec
            }
        }
        &Variable::F64(_, None) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "This does not make sense, perhaps an array is empty?".into()
            });
        }
        x => return Err(rt.expected_arg(0, x, "f64")),
    })))
}

pub(crate) fn explain_why(rt: &mut Runtime) -> Result<Variable, String> {
    let why = rt.stack.pop().expect(TINVOTS);
    let val = rt.stack.pop().expect(TINVOTS);
    let (val, why) = match rt.get(&val) {
        &Variable::Bool(val, ref sec) => (
            val,
            match *sec {
                None => Box::new(vec![deep_clone(&why, &rt.stack)]),
                Some(ref sec) => {
                    let mut sec = sec.clone();
                    sec.push(deep_clone(&why, &rt.stack));
                    sec
                }
            },
        ),
        x => return Err(rt.expected_arg(0, x, "bool")),
    };
    Ok(Variable::Bool(val, Some(why)))
}

pub(crate) fn explain_where(rt: &mut Runtime) -> Result<Variable, String> {
    let wh = rt.stack.pop().expect(TINVOTS);
    let val = rt.stack.pop().expect(TINVOTS);
    let (val, wh) = match rt.get(&val) {
        &Variable::F64(val, ref sec) => (
            val,
            match *sec {
                None => Box::new(vec![deep_clone(&wh, &rt.stack)]),
                Some(ref sec) => {
                    let mut sec = sec.clone();
                    sec.push(deep_clone(&wh, &rt.stack));
                    sec
                }
            },
        ),
        x => return Err(rt.expected_arg(0, x, "bool")),
    };
    Ok(Variable::F64(val, Some(wh)))
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
dyon_fn! {fn sleep(v: f64) {
    use std::thread::sleep;
    use std::time::Duration;

    let secs = v as u64;
    let nanos = (v.fract() * 1.0e9) as u32;
    sleep(Duration::new(secs, nanos));
}}

pub(crate) fn head(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Option(match rt.get(&v) {
        &Variable::Link(ref link) => link.head(),
        x => return Err(rt.expected_arg(0, x, "link")),
    }))
}

pub(crate) fn tip(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Option(match rt.get(&v) {
        &Variable::Link(ref link) => link.tip(),
        x => return Err(rt.expected_arg(0, x, "link")),
    }))
}

pub(crate) fn tail(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Link(Box::new(match rt.get(&v) {
        &Variable::Link(ref link) => link.tail(),
        x => return Err(rt.expected_arg(0, x, "link")),
    })))
}

pub(crate) fn neck(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Link(Box::new(match rt.get(&v) {
        &Variable::Link(ref link) => link.neck(),
        x => return Err(rt.expected_arg(0, x, "link")),
    })))
}

pub(crate) fn is_empty(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::bool(match rt.get(&v) {
        &Variable::Link(ref link) => link.is_empty(),
        x => return Err(rt.expected_arg(0, x, "link")),
    }))
}

pub(crate) fn random(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(Variable::f64(rand::random()))
}

dyon_fn! {fn tau() -> f64 {6.283_185_307_179_586}}

pub(crate) fn len(a: &Variable) -> Result<Variable, String> {
    match a {
        Variable::Array(arr) => Ok(Variable::f64(arr.len() as f64)),
        _ => Err("Expected array".into()),
    }
}

pub(crate) fn push_ref(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).push(item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn insert_ref(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let index = rt.stack.pop().expect(TINVOTS);
    let index = match rt.get(&index) {
        &Variable::F64(index, _) => index,
        x => return Err(rt.expected_arg(1, x, "number")),
    };
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        if let Variable::Array(ref arr) = rt.stack[ind] {
            let index = index as usize;
            if index > arr.len() {
                return Err("Index out of bounds".into());
            }
        }
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).insert(index as usize, item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn push(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let item = deep_clone(rt.get(&item), &rt.stack);
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).push(item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn insert(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let item = deep_clone(rt.get(&item), &rt.stack);
    let index = rt.stack.pop().expect(TINVOTS);
    let index = match rt.get(&index) {
        &Variable::F64(index, _) => index,
        x => return Err(rt.expected_arg(1, x, "number")),
    };
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        if let Variable::Array(ref arr) = rt.stack[ind] {
            let index = index as usize;
            if index > arr.len() {
                return Err("Index out of bounds".into());
            }
        }
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).insert(index as usize, item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn pop(rt: &mut Runtime) -> Result<Variable, String> {
    let arr = rt.stack.pop().expect(TINVOTS);
    let mut v: Option<Variable> = None;
    if let Variable::Ref(ind) = arr {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            v = Arc::make_mut(arr).pop();
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    let v = match v {
        None => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected non-empty array".into()
            });
        }
        Some(val) => val,
    };
    Ok(v)
}

pub(crate) fn remove(rt: &mut Runtime) -> Result<Variable, String> {
    let index = rt.stack.pop().expect(TINVOTS);
    let index = match rt.get(&index) {
        &Variable::F64(index, _) => index,
        x => return Err(rt.expected_arg(1, x, "number")),
    };
    let arr = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = arr {
        if let Variable::Array(ref arr) = rt.stack[ind] {
            let index = index as usize;
            if index >= arr.len() {
                return Err("Index out of bounds".into());
            }
        }
        if let Variable::Array(ref mut arr) = rt.stack[ind] {
            let v = Arc::make_mut(arr).remove(index as usize);
            return Ok(v);
        };
        Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    } else {
        Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
}

pub(crate) fn reverse(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).reverse();
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn clear(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).clear();
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn swap(rt: &mut Runtime) -> Result<(), String> {
    let j = rt.stack.pop().expect(TINVOTS);
    let i = rt.stack.pop().expect(TINVOTS);
    let j = match rt.get(&j) {
        &Variable::F64(val, _) => val,
        x => return Err(rt.expected_arg(2, x, "number")),
    };
    let i = match rt.get(&i) {
        &Variable::F64(val, _) => val,
        x => return Err(rt.expected_arg(1, x, "number")),
    };
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).swap(i as usize, j as usize);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            });
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        });
    }
    Ok(())
}

pub(crate) fn _typeof(rt: &mut Runtime) -> Result<Variable, String> {
    use dyon::Variable::*;

    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Str(match *rt.get(&v) {
        Str(_) => TEXT_TYPE.clone(),
        F64(_, _) => F64_TYPE.clone(),
        Vec4(_) => VEC4_TYPE.clone(),
        Mat4(_) => MAT4_TYPE.clone(),
        Return => RETURN_TYPE.clone(),
        Bool(_, _) => BOOL_TYPE.clone(),
        Object(_) => OBJECT_TYPE.clone(),
        Array(_) => ARRAY_TYPE.clone(),
        Link(_) => LINK_TYPE.clone(),
        Ref(_) => REF_TYPE.clone(),
        UnsafeRef(_) => UNSAFE_REF_TYPE.clone(),
        RustObject(_) => RUST_OBJECT_TYPE.clone(),
        Option(_) => OPTION_TYPE.clone(),
        Result(_) => RESULT_TYPE.clone(),
        Thread(_) => THREAD_TYPE.clone(),
        Closure(_, _) => CLOSURE_TYPE.clone(),
        In(_) => IN_TYPE.clone(),
    }))
}

pub(crate) fn load__source_imports(rt: &mut Runtime) -> Result<Variable, String> {
    use dyon::load;

    let modules = rt.stack.pop().expect(TINVOTS);
    let source = rt.stack.pop().expect(TINVOTS);
    let mut new_module = Module::empty();
    new_module.import_ext_prelude(&rt.module);
    new_module.import_tr_prelude(&rt.module);
    let x = rt.get(&modules);
    match x {
        &Variable::Array(ref array) => {
            for it in &**array {
                match rt.get(it) {
                    &Variable::RustObject(ref obj) => {
                        match obj.lock().unwrap().downcast_ref::<Arc<Module>>() {
                            Some(m) => new_module.import(m),
                            None => return Err(rt.expected_arg(1, x, "[Module]")),
                        }
                    }
                    x => return Err(rt.expected_arg(1, x, "[Module]")),
                }
            }
        }
        x => return Err(rt.expected_arg(1, x, "[Module]")),
    }
    Ok(match rt.get(&source) {
        &Variable::Str(ref text) => {
            if let Err(err) = load(text, &mut new_module) {
                Variable::Result(Err(Box::new(Error {
                    message: Variable::Str(Arc::new(format!(
                        "When attempting to load module:\n{}",
                        err
                    ))),
                    trace: vec![],
                })))
            } else {
                Variable::Result(Ok(Box::new(Variable::RustObject(Arc::new(Mutex::new(
                    Arc::new(new_module),
                ))))))
            }
        }
        x => return Err(rt.expected_arg(0, x, "str")),
    })
}

pub(crate) fn module__in_string_imports(rt: &mut Runtime) -> Result<Variable, String> {
    let modules = rt.stack.pop().expect(TINVOTS);
    let source = rt.stack.pop().expect(TINVOTS);
    let source = match rt.get(&source) {
        &Variable::Str(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str")),
    };
    let name = rt.stack.pop().expect(TINVOTS);
    let name = match rt.get(&name) {
        &Variable::Str(ref t) => t.clone(),
        x => return Err(rt.expected_arg(0, x, "str")),
    };
    let mut new_module = Module::empty();
    new_module.import_ext_prelude(&rt.module);
    new_module.import_tr_prelude(&rt.module);
    let x = rt.get(&modules);
    match x {
        &Variable::Array(ref array) => {
            for it in &**array {
                match rt.get(it) {
                    &Variable::RustObject(ref obj) => {
                        match obj.lock().unwrap().downcast_ref::<Arc<Module>>() {
                            Some(m) => new_module.import(m),
                            None => return Err(rt.expected_arg(2, x, "[Module]")),
                        }
                    }
                    x => return Err(rt.expected_arg(2, x, "[Module]")),
                }
            }
        }
        x => return Err(rt.expected_arg(2, x, "[Module]")),
    }
    Ok(if let Err(err) = load_str(&name, source, &mut new_module) {
        Variable::Result(Err(Box::new(Error {
            message: Variable::Str(Arc::new(format!(
                "When attempting to load module:\n{}",
                err
            ))),
            trace: vec![],
        })))
    } else {
        Variable::Result(Ok(Box::new(Variable::RustObject(Arc::new(Mutex::new(
            Arc::new(new_module),
        ))))))
    })
}

dyon_fn! {fn none() -> Variable {Variable::Option(None)}}

pub(crate) fn some(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Option(Some(Box::new(deep_clone(
        rt.get(&v),
        &rt.stack,
    )))))
}

pub(crate) fn ok(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Result(Ok(Box::new(deep_clone(rt.get(&v), &rt.stack))));
    Ok(v)
}

pub(crate) fn err(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Result(Err(Box::new(Error {
        message: deep_clone(rt.get(&v), &rt.stack),
        trace: vec![],
    }))))
}

pub(crate) fn is_err(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        &Variable::Result(Err(_)) => Variable::bool(true),
        &Variable::Result(Ok(_)) => Variable::bool(false),
        x => return Err(rt.expected_arg(0, x, "result")),
    })
}

pub(crate) fn is_ok(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        Variable::Result(Err(_)) => Variable::bool(false),
        Variable::Result(Ok(_)) => Variable::bool(true),
        x => return Err(rt.expected_arg(0, x, "result")),
    })
}

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

pub(crate) fn unwrap(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        &Variable::Option(Some(ref v)) => (**v).clone(),
        &Variable::Option(None) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected `some(_)`".into()
            });
        }
        &Variable::Result(Ok(ref ok)) => (**ok).clone(),
        &Variable::Result(Err(ref err)) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                format!("Error Unwrapping the variable: {:?}!", err.message).to_string()
            });
        }
        x => return Err(rt.expected_arg(0, x, "some(_) or ok(_)")),
    })
}

pub(crate) fn unwrap_or(rt: &mut Runtime) -> Result<Variable, String> {
    // Return value does not depend on lifetime of argument since
    // `ok(x)` and `some(x)` perform a deep clone.
    let def = rt.stack.pop().expect(TINVOTS);
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        &Variable::Option(Some(ref v)) => (**v).clone(),
        &Variable::Result(Ok(ref ok)) => (**ok).clone(),
        &Variable::Option(None) | &Variable::Result(Err(_)) => rt.get(&def).clone(),
        x => return Err(rt.expected_arg(0, x, "some(_) or ok(_)")),
    })
}

pub(crate) fn unwrap_err(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        &Variable::Result(Err(ref err)) => err.message.clone(),
        x => return Err(rt.expected_arg(0, x, "err(_)")),
    })
}

pub(crate) fn join__thread(rt: &mut Runtime) -> Result<Variable, String> {
    let thread = rt.stack.pop().expect(TINVOTS);
    let handle_res = Thread::invalidate_handle(rt, thread);
    Ok(Variable::Result({
        match handle_res {
            Ok(handle) => match join!(rt.tokio_runtime, handle) {
                Ok(res) => match res {
                    Ok(res) => Ok(Box::new(res)),
                    Err(err) => Err(Box::new(Error {
                        message: Variable::Str(Arc::new(err)),
                        trace: vec![],
                    })),
                },
                Err(_err) => Err(Box::new(Error {
                    message: Variable::Str(Arc::new("Thread did not exit successfully".into())),
                    trace: vec![],
                })),
            },
            Err(err) => Err(Box::new(Error {
                message: Variable::Str(Arc::new(err)),
                trace: vec![],
            })),
        }
    }))
}

pub(crate) fn has(rt: &mut Runtime) -> Result<Variable, String> {
    let key = rt.stack.pop().expect(TINVOTS);
    let key = match rt.get(&key) {
        &Variable::Str(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str")),
    };
    let obj = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::bool(match rt.get(&obj) {
        &Variable::Object(ref obj) => obj.contains_key(&key),
        x => return Err(rt.expected_arg(0, x, "object")),
    }))
}

pub(crate) fn keys(rt: &mut Runtime) -> Result<Variable, String> {
    let obj = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Array(Arc::new(match rt.get(&obj) {
        &Variable::Object(ref obj) => obj.keys().map(|k| Variable::Str(k.clone())).collect(),
        x => return Err(rt.expected_arg(0, x, "object")),
    })))
}

pub(crate) fn chars(rt: &mut Runtime) -> Result<Variable, String> {
    let t = rt.stack.pop().expect(TINVOTS);
    let t = match rt.get(&t) {
        &Variable::Str(ref t) => t.clone(),
        x => return Err(rt.expected_arg(0, x, "str")),
    };
    Ok(Variable::Array(Arc::new(
        t.chars()
            .map(|ch| {
                let mut s = String::new();
                s.push(ch);
                Variable::Str(Arc::new(s))
            })
            .collect::<Vec<_>>(),
    )))
}

dyon_fn! {fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => val.as_secs() as f64 +
                   f64::from(val.subsec_nanos()) / 1.0e9,
        Err(err) => -{
            let val = err.duration();
            val.as_secs() as f64 +
            f64::from(val.subsec_nanos()) / 1.0e9
        }
    }
}}

dyon_fn! {fn is_nan(v: f64) -> bool {v.is_nan()}}

pub(crate) fn wait_next(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        &Variable::In(ref mutex) => match mutex.lock() {
            Ok(x) => match x.recv() {
                Ok(x) => Variable::Option(Some(Box::new(x))),
                Err(_) => Variable::Option(None),
            },
            Err(err) => return Err(format!("Can not lock In mutex:\n{}", err.to_string())),
        },
        x => return Err(rt.expected_arg(0, x, "in")),
    })
}

pub(crate) fn next(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(match rt.get(&v) {
        &Variable::In(ref mutex) => match mutex.lock() {
            Ok(x) => match x.try_recv() {
                Ok(x) => Variable::Option(Some(Box::new(x))),
                Err(_) => Variable::Option(None),
            },
            Err(err) => return Err(format!("Can not lock In mutex:\n{}", err.to_string())),
        },
        x => return Err(rt.expected_arg(0, x, "in")),
    })
}

pub fn add_functions(module: &mut Module) {
    use dyon::Type::*;

    module.ns("std");

    add_boolean_operations(module);
    add_mat_operations(module);

    add_math_functions(module);

    module.add_str("clone", clone, Dfn::nl(vec![Any], Any));

    module.add_str("typeof", _typeof, Dfn::nl(vec![Any], Str));
    module.add_str("none", none, Dfn::nl(vec![], Type::option()));
    module.add_str("some", some, Dfn::nl(vec![Any], Type::option()));
    module.add_str("ok", ok, Dfn::nl(vec![Any], Type::result()));
    module.add_str("err", err, Dfn::nl(vec![Any], Type::result()));

    module.add_str(
        "join__thread",
        join__thread,
        Dfn::nl(vec![Type::thread()], Result(Box::new(Any))),
    );

    module.add_str("now", now, Dfn::nl(vec![], F64));
    module.add_str(
        "load__source_imports",
        load__source_imports,
        Dfn::nl(vec![Str, Type::array()], Type::result()),
    );
    module.add_str(
        "module__in_string_imports",
        module__in_string_imports,
        Dfn::nl(vec![Str, Str, Type::array()], Type::result()),
    );
    module.add_str("is_err", is_err, Dfn::nl(vec![Type::result()], Bool));
    module.add_str("is_ok", is_ok, Dfn::nl(vec![Type::result()], Bool));

    module.add_str("unwrap", unwrap, Dfn::nl(vec![Any], Any));
    module.add_str(
        "why",
        why,
        Dfn::nl(vec![Secret(Box::new(Bool))], Type::array()),
    );
    module.add_str(
        "where",
        _where,
        Dfn::nl(vec![Secret(Box::new(F64))], Type::array()),
    );
    module.add_str(
        "explain_why",
        explain_why,
        Dfn::nl(vec![Bool, Any], Secret(Box::new(Bool))),
    );
    module.add_str(
        "explain_where",
        explain_where,
        Dfn::nl(vec![F64, Any], Secret(Box::new(F64))),
    );
    module.add_str("head", head, Dfn::nl(vec![Link], Any));
    module.add_str("tip", tip, Dfn::nl(vec![Link], Option(Box::new(Any))));
    module.add_str("tail", tail, Dfn::nl(vec![Link], Link));
    module.add_str("neck", neck, Dfn::nl(vec![Link], Link));
    module.add_str("is_empty", is_empty, Dfn::nl(vec![Link], Bool));
    module.add_unop_str("len", len, Dfn::nl(vec![Type::array()], F64));
    module.add_str(
        "push_ref(mut,_)",
        push_ref,
        Dfn {
            lts: vec![Lt::Default, Lt::Arg(0)],
            tys: vec![Type::array(), Any],
            ret: Void,
            ext: vec![],
            lazy: LAZY_NO,
        },
    );
    module.add_str(
        "insert_ref(mut,_,_)",
        insert_ref,
        Dfn {
            lts: vec![Lt::Default, Lt::Default, Lt::Arg(0)],
            tys: vec![Type::array(), F64, Any],
            ret: Void,
            ext: vec![],
            lazy: LAZY_NO,
        },
    );
    module.add_str("push(mut,_)", push, Dfn::nl(vec![Type::array(), Any], Void));
    module.add_str(
        "insert(mut,_,_)",
        insert,
        Dfn {
            lts: vec![Lt::Default; 3],
            tys: vec![Type::array(), F64, Any],
            ret: Void,
            ext: vec![],
            lazy: LAZY_NO,
        },
    );
    module.add_str(
        "pop(mut)",
        pop,
        Dfn {
            lts: vec![Lt::Return],
            tys: vec![Type::array()],
            ret: Any,
            ext: vec![],
            lazy: LAZY_NO,
        },
    );
    module.add_str(
        "remove(mut,_)",
        remove,
        Dfn {
            lts: vec![Lt::Return, Lt::Default],
            tys: vec![Type::array(), F64],
            ret: Any,
            ext: vec![],
            lazy: LAZY_NO,
        },
    );
    module.add_str("reverse(mut)", reverse, Dfn::nl(vec![Type::array()], Void));
    module.add_str("clear(mut)", clear, Dfn::nl(vec![Type::array()], Void));
    module.add_str(
        "swap(mut,_,_)",
        swap,
        Dfn::nl(vec![Type::array(), F64, F64], Void),
    );
    module.add_str(
        "unwrap_or",
        unwrap_or,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![],
            lazy: LAZY_UNWRAP_OR,
        },
    );
    module.add_str("unwrap_err", unwrap_err, Dfn::nl(vec![Any], Any));
    module.add_str("has", has, Dfn::nl(vec![Object, Str], Bool));
    module.add_str("keys", keys, Dfn::nl(vec![Object], Array(Box::new(Str))));
    module.add_str("chars", chars, Dfn::nl(vec![Str], Array(Box::new(Str))));

    module.add_str(
        "wait_next",
        wait_next,
        Dfn::nl(vec![In(Box::new(Any))], Option(Box::new(Str))),
    );
    module.add_str(
        "next",
        next,
        Dfn::nl(vec![In(Box::new(Any))], Option(Box::new(Str))),
    );
}

pub fn add_mat_operations(module: &mut Module) {
    use dyon::Type::{Mat4, Vec4};

    module.add_binop(
        ADD.clone(),
        add,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![
                Type::all_ext(vec![F64, F64], F64),
                Type::all_ext(vec![Vec4, Vec4], Vec4),
                Type::all_ext(vec![Vec4, F64], Vec4),
                Type::all_ext(vec![F64, Vec4], Vec4),
                Type::all_ext(vec![Mat4, Mat4], Mat4),
                Type::all_ext(vec![F64, Mat4], Mat4),
                Type::all_ext(vec![Mat4, F64], Mat4),
                Type::all_ext(vec![Bool, Bool], Bool),
                Type::all_ext(vec![Str, Str], Str),
                Type::all_ext(vec![Link, Link], Link),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        SUB.clone(),
        sub,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![
                Type::all_ext(vec![F64, F64], F64),
                Type::all_ext(vec![Vec4, Vec4], Vec4),
                Type::all_ext(vec![Vec4, F64], Vec4),
                Type::all_ext(vec![F64, Vec4], Vec4),
                Type::all_ext(vec![Mat4, Mat4], Mat4),
                Type::all_ext(vec![F64, Mat4], Mat4),
                Type::all_ext(vec![Mat4, F64], Mat4),
                Type::all_ext(vec![Bool, Bool], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        MUL.clone(),
        mul,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![
                (vec![], vec![F64, F64], F64),
                (vec![], vec![Vec4, Vec4], Vec4),
                (vec![], vec![Vec4, F64], Vec4),
                (vec![], vec![F64, Vec4], Vec4),
                (vec![], vec![Mat4, Mat4], Mat4),
                (vec![], vec![F64, Mat4], Mat4),
                (vec![], vec![Mat4, F64], Mat4),
                (vec![], vec![Mat4, Vec4], Vec4),
                Type::all_ext(vec![Bool, Bool], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        DIV.clone(),
        div,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![
                (vec![], vec![F64, F64], F64),
                (vec![], vec![Vec4, Vec4], Vec4),
                (vec![], vec![Vec4, F64], Vec4),
                (vec![], vec![F64, Vec4], Vec4),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        REM.clone(),
        rem,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![
                (vec![], vec![F64, F64], F64),
                (vec![], vec![Vec4, Vec4], Vec4),
                (vec![], vec![Vec4, F64], Vec4),
                (vec![], vec![F64, Vec4], Vec4),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        POW.clone(),
        pow,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: Any,
            ext: vec![
                (vec![], vec![F64, F64], F64),
                (vec![], vec![Vec4, Vec4], Vec4),
                (vec![], vec![Vec4, F64], Vec4),
                (vec![], vec![F64, Vec4], Vec4),
                Type::all_ext(vec![Bool, Bool], Bool),
            ],
            lazy: LAZY_NO,
        },
    );

    module.add_unop(
        NEG.clone(),
        neg,
        Dfn {
            lts: vec![Lt::Default],
            tys: vec![Any],
            ret: Any,
            ext: vec![
                (vec![], vec![F64], F64),
                (vec![], vec![Vec4], Vec4),
                (vec![], vec![Mat4], Mat4),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        DOT.clone(),
        dot,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any; 2],
            ret: F64,
            ext: vec![
                (vec![], vec![Vec4, Vec4], F64),
                (vec![], vec![Vec4, F64], F64),
                (vec![], vec![F64, Vec4], F64),
            ],
            lazy: LAZY_NO,
        },
    );
}

pub fn add_boolean_operations(module: &mut Module) {
    use dyon::Type::Vec4;

    module.add_binop(
        LESS.clone(),
        less,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any, Any],
            ret: Bool,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(F64)), F64],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![F64; 2], Bool),
                (vec![], vec![Str; 2], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        LESS_OR_EQUAL.clone(),
        less_or_equal,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any, Any],
            ret: Bool,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(F64)), F64],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![F64; 2], Bool),
                (vec![], vec![Str; 2], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        GREATER.clone(),
        greater,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any, Any],
            ret: Bool,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(F64)), F64],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![F64; 2], Bool),
                (vec![], vec![Str; 2], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        GREATER_OR_EQUAL.clone(),
        greater_or_equal,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any, Any],
            ret: Bool,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(F64)), F64],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![F64; 2], Bool),
                (vec![], vec![Str; 2], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        EQUAL.clone(),
        equal,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any, Any],
            ret: Bool,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(F64)), F64],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![F64; 2], Bool),
                (vec![], vec![Str; 2], Bool),
                (
                    vec![],
                    vec![Secret(Box::new(Bool)), Bool],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![Bool; 2], Bool),
                (vec![], vec![Vec4; 2], Bool),
                (vec![], vec![Type::object(), Type::object()], Bool),
                (vec![], vec![Type::array(), Type::array()], Bool),
                (vec![], vec![Type::option(), Type::option()], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add_binop(
        NOT_EQUAL.clone(),
        not_equal,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Any, Any],
            ret: Bool,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(F64)), F64],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![F64; 2], Bool),
                (vec![], vec![Str; 2], Bool),
                (
                    vec![],
                    vec![Secret(Box::new(Bool)), Bool],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![Bool; 2], Bool),
                (vec![], vec![Vec4; 2], Bool),
                (vec![], vec![Type::object(), Type::object()], Bool),
                (vec![], vec![Type::array(), Type::array()], Bool),
                (vec![], vec![Type::option(), Type::option()], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
    module.add(
        AND_ALSO.clone(),
        and_also,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Bool, Bool],
            ret: Any,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(Bool)), Bool],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![Bool; 2], Bool),
            ],
            lazy: LAZY_AND,
        },
    );
    module.add(
        OR_ELSE.clone(),
        or_else,
        Dfn {
            lts: vec![Lt::Default; 2],
            tys: vec![Bool, Bool],
            ret: Any,
            ext: vec![
                (
                    vec![],
                    vec![Secret(Box::new(Bool)), Bool],
                    Secret(Box::new(Bool)),
                ),
                (vec![], vec![Bool; 2], Bool),
            ],
            lazy: LAZY_OR,
        },
    );
    module.add_unop(
        NOT.clone(),
        not,
        Dfn {
            lts: vec![Lt::Default],
            tys: vec![Any],
            ret: Any,
            ext: vec![
                (vec![], vec![Secret(Box::new(Bool))], Secret(Box::new(Bool))),
                (vec![], vec![Bool], Bool),
            ],
            lazy: LAZY_NO,
        },
    );
}

pub fn add_math_functions(module: &mut Module) {
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
    module.add_str("sleep", sleep, Dfn::nl(vec![F64], Void));
    #[cfg(not(target_family = "wasm"))]
    module.add_str("random", random, Dfn::nl(vec![], F64));
    module.add_str("tau", tau, Dfn::nl(vec![], F64));
    module.add_str("is_nan", is_nan, Dfn::nl(vec![F64], Bool));
    module.add_str("min", min, Dfn::nl(vec![Array(Box::new(F64))], F64));
    module.add_str("max", max, Dfn::nl(vec![Array(Box::new(F64))], F64));
}
