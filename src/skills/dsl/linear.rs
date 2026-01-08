use dyon::Type::*;
use dyon::{Dfn, Mat4, Module, Runtime, Variable, Vec4};
use std::f64::consts::PI;
use std::result::Result;

use crate::skills::dsl::std::TINVOTS;

pub fn add_functions(module: &mut Module) {
    use dyon::Type::{Mat4, Vec4};

    module.ns("linear");

    module.add_str(
        "rot__axis_angle",
        rot__axis_angle,
        Dfn::nl(vec![Vec4, F64], Mat4),
    );
    module.add_str(
        "ortho__pos_right_up_forward",
        ortho__pos_right_up_forward,
        Dfn::nl(vec![Vec4; 4], Mat4),
    );
    module.add_str(
        "proj__fov_near_far_ar",
        proj__fov_near_far_ar,
        Dfn::nl(vec![F64; 4], Mat4),
    );
    module.add_str(
        "mvp__model_view_projection",
        mvp__model_view_projection,
        Dfn::nl(vec![Mat4; 3], Mat4),
    );
    module.add_str("cross", cross, Dfn::nl(vec![Vec4, Vec4], Vec4));
    module.add_str("x", x, Dfn::nl(vec![Vec4], F64));
    module.add_str("y", y, Dfn::nl(vec![Vec4], F64));
    module.add_str("z", z, Dfn::nl(vec![Vec4], F64));
    module.add_str("w", w, Dfn::nl(vec![Vec4], F64));
    module.add_unop_str("norm", norm, Dfn::nl(vec![Vec4], F64));
    module.add_str("rv", rv, Dfn::nl(vec![Mat4, F64], Vec4));
    module.add_str("s", s, Dfn::nl(vec![Vec4, F64], F64));
    module.add_str("det", det, Dfn::nl(vec![Mat4], F64));
    module.add_str("inv", inv, Dfn::nl(vec![Mat4], Mat4));
    module.add_str("mov", mov, Dfn::nl(vec![Vec4], Mat4));
    module.add_str("scale", scale, Dfn::nl(vec![Vec4], Mat4));
    module.add_str("rx", rx, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("ry", ry, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("rz", rz, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("rw", rw, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("cx", cx, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("cy", cy, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("cz", cz, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("cw", cw, Dfn::nl(vec![Mat4], Vec4));
    module.add_str("cv", cv, Dfn::nl(vec![Mat4, F64], Vec4));
    module.add_str("dir__angle", dir__angle, Dfn::nl(vec![F64], Vec4));
}

dyon_fn! {fn dir__angle(val: f64) -> Vec4 {Vec4([val.cos() as f32, val.sin() as f32, 0.0, 0.0])}}

dyon_fn! {fn cross(a: Vec4, b: Vec4) -> Vec4 {
    Vec4([a.0[1] * b.0[2] - a.0[2] * b.0[1],
          a.0[2] * b.0[0] - a.0[0] * b.0[2],
          a.0[0] * b.0[1] - a.0[1] * b.0[0], 0.0])
}}

dyon_fn! {fn x(v: Vec4) -> f64 {f64::from(v.0[0])}}
dyon_fn! {fn y(v: Vec4) -> f64 {f64::from(v.0[1])}}
dyon_fn! {fn z(v: Vec4) -> f64 {f64::from(v.0[2])}}
dyon_fn! {fn w(v: Vec4) -> f64 {f64::from(v.0[3])}}

pub(crate) fn norm(v: &Variable) -> Result<Variable, String> {
    if let Variable::Vec4(v) = *v {
        Ok(Variable::f64(vecmath::vec4_len(v) as f64))
    } else {
        Err("Expected `vec4`".into())
    }
}

pub(crate) fn s(rt: &mut Runtime) -> Result<Variable, String> {
    let ind: f64 = rt.pop().expect(TINVOTS);
    let ind = ind as usize;
    if ind >= 4 {
        return Err(format!("Index out of bounds `{}`", ind));
    };
    let v: [f32; 4] = rt.pop_vec4().expect(TINVOTS);
    Ok(Variable::f64(f64::from(v[ind])))
}

dyon_fn! {fn det(m: Mat4) -> f64 {f64::from(vecmath::mat4_det(m.0))}}
dyon_fn! {fn inv(m: Mat4) -> Mat4 {Mat4(vecmath::mat4_inv(m.0))}}
dyon_fn! {fn mov(v: Vec4) -> Mat4 {Mat4([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [v.0[0], v.0[1], v.0[2], 1.0],
])}}
dyon_fn! {fn rot__axis_angle(axis: Vec4, ang: f64) -> Mat4 {
    let axis = [f64::from(axis.0[0]), f64::from(axis.0[1]), f64::from(axis.0[2])];
    let cos = ang.cos();
    let sin = ang.sin();
    let inv_cos = 1.0 - cos;
    Mat4([
        [
            (cos + axis[0] * axis[0] * inv_cos) as f32,
            (axis[0] * axis[1] * inv_cos - axis[2] * sin) as f32,
            (axis[0] * axis[2] * inv_cos + axis[1] * sin) as f32,
            0.0
        ],
        [
            (axis[1] * axis[0] * inv_cos + axis[2] * sin) as f32,
            (cos + axis[1] * axis[1] * inv_cos) as f32,
            (axis[1] * axis[2] * inv_cos - axis[0] * sin) as f32,
            0.0
        ],
        [
            (axis[2] * axis[0] * inv_cos - axis[1] * sin) as f32,
            (axis[2] * axis[1] * inv_cos + axis[0] * sin) as f32,
            (cos + axis[2] * axis[2] * inv_cos) as f32,
            0.0
        ],
        [0.0,0.0,0.0,1.0]
    ])
}}
dyon_fn! {fn ortho__pos_right_up_forward(pos: Vec4, right: Vec4, up: Vec4, forward: Vec4) -> Mat4 {
    use vecmath::vec4_dot as dot;
    Mat4([
        [right.0[0], up.0[0], forward.0[0], 0.0],
        [right.0[1], up.0[1], forward.0[1], 0.0],
        [right.0[2], up.0[2], forward.0[2], 0.0],
        [-dot(right.0, pos.0), -dot(up.0, pos.0), -dot(forward.0, pos.0), 1.0],
    ])
}}
dyon_fn! {fn proj__fov_near_far_ar(fov: f64, near: f64, far: f64, ar: f64) -> Mat4 {
    let f = 1.0 / (fov * PI).tan();
    Mat4([
        [(f/ar) as f32, 0.0, 0.0, 0.0],
        [0.0, f as f32, 0.0, 0.0],
        [0.0, 0.0, ((far + near) / (near - far)) as f32, -1.0],
        [0.0, 0.0, ((2.0 * far * near) / (near - far)) as f32, 0.0],
    ])
}}
dyon_fn! {fn mvp__model_view_projection(model: Mat4, view: Mat4, proj: Mat4) -> Mat4 {
    use vecmath::col_mat4_mul as mul;
    Mat4(mul(mul(proj.0, view.0), model.0))
}}
dyon_fn! {fn scale(v: Vec4) -> Mat4 {Mat4([
    [v.0[0], 0.0, 0.0, 0.0],
    [0.0, v.0[1], 0.0, 0.0],
    [0.0, 0.0, v.0[2], 0.0],
    [0.0, 0.0, 0.0, 1.0],
])}}

dyon_fn! {fn rx(m: Mat4) -> Vec4 {Vec4([m.0[0][0], m.0[1][0], m.0[2][0], m.0[3][0]])}}
dyon_fn! {fn ry(m: Mat4) -> Vec4 {Vec4([m.0[0][1], m.0[1][1], m.0[2][1], m.0[3][1]])}}
dyon_fn! {fn rz(m: Mat4) -> Vec4 {Vec4([m.0[0][2], m.0[1][2], m.0[2][2], m.0[3][2]])}}
dyon_fn! {fn rw(m: Mat4) -> Vec4 {Vec4([m.0[0][3], m.0[1][3], m.0[2][3], m.0[3][3]])}}

pub(crate) fn rv(rt: &mut Runtime) -> Result<Variable, String> {
    let ind: f64 = rt.pop().expect(TINVOTS);
    let ind = ind as usize;
    if ind >= 4 {
        return Err(format!("Index out of bounds `{}`", ind));
    };
    let m: [[f32; 4]; 4] = rt.pop_mat4().expect(TINVOTS);
    Ok(Variable::Vec4([m[0][ind], m[1][ind], m[2][ind], m[3][ind]]))
}

dyon_fn! {fn cx(m: Mat4) -> Vec4 {Vec4(m.0[0])}}
dyon_fn! {fn cy(m: Mat4) -> Vec4 {Vec4(m.0[1])}}
dyon_fn! {fn cz(m: Mat4) -> Vec4 {Vec4(m.0[2])}}
dyon_fn! {fn cw(m: Mat4) -> Vec4 {Vec4(m.0[3])}}

pub(crate) fn cv(rt: &mut Runtime) -> Result<Variable, String> {
    let ind: f64 = rt.pop().expect(TINVOTS);
    let ind = ind as usize;
    if ind >= 4 {
        return Err(format!("Index out of bounds `{}`", ind));
    };
    let m: [[f32; 4]; 4] = rt.pop_mat4().expect(TINVOTS);
    Ok(Variable::Mat4(Box::new(m)))
}
