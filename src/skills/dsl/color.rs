use dyon::{Dfn, Module, Type};
use std::result::Result;

pub fn add_functions(module: &mut Module) {
    module.ns("color");
    module.add_str(
        "srgb_to_linear__color",
        srgb_to_linear__color,
        Dfn::nl(vec![Type::Vec4], Type::Vec4),
    );
    module.add_str(
        "linear_to_srgb__color",
        linear_to_srgb__color,
        Dfn::nl(vec![Type::Vec4], Type::Vec4),
    );
}

dyon_fn! {fn srgb_to_linear__color(v: dyon::Vec4) ->  dyon::Vec4 {
    let v = v.0;
    let to_linear = |f: f32| {
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    };
    dyon::Vec4([to_linear(v[0]), to_linear(v[1]), to_linear(v[2]), v[3]])
}}

dyon_fn! {fn linear_to_srgb__color(v: dyon::Vec4) ->  dyon::Vec4 {
    let v = v.0;
    let to_srgb = |f: f32| {
        if f <= 0.003_130_8 {
            f * 12.92
        } else {
            1.055 * f.powf(1.0 / 2.4) - 0.055
        }
    };
    dyon::Vec4([to_srgb(v[0]), to_srgb(v[1]), to_srgb(v[2]), v[3]])
}}
