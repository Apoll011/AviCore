use crate::skills::dsl::dyon_helpers::deep_clone;
use crate::skills::dsl::std::TINVOTS;
use dyon::Type::{Any, F64, Void};
use dyon::{Dfn, LAZY_NO, Lt, Module, Runtime, Type, Variable};
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("array");

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
}

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
