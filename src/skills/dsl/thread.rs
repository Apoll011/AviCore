use std::sync::Arc;
use dyon::{Dfn, Error, Module, Runtime, Thread, Type, Variable};
use dyon::Type::{Any, In, Option, Str, Void, F64};
use crate::skills::dsl::std::{TINVOTS};

pub fn add_functions(module: &mut Module) {
    module.ns("thread");
    module.add_str(
        "join__thread",
        join__thread,
        Dfn::nl(vec![Type::thread()], Type::Result(Box::new(Any))),
    );
    module.add_str("sleep", sleep, Dfn::nl(vec![F64], Void));

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

#[allow(non_snake_case)]
pub fn join__thread(rt: &mut Runtime) -> Result<Variable, String> {
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

dyon_fn! {fn sleep(v: f64) {
    use std::thread::sleep;
    use std::time::Duration;

    let secs = v as u64;
    let nanos = (v.fract() * 1.0e9) as u32;
    sleep(Duration::new(secs, nanos));
}}

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
