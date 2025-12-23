use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use crate::intent::YamlValue;
use serde_yaml::Value as Yaml;
use crate::skills::skill_context::{Setting, SettingNamed};
use super::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("setting");
    module.add(Arc::new("get".into()), get_setting, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("full".into()), get_setting_full, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("list".into()), list_settings, Dfn::nl(vec![], Any));
    module.add(Arc::new("has".into()), has_setting, Dfn::nl(vec![Str], Any));
}

#[allow(non_snake_case)]
pub fn get_setting(_rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = _rt.pop()?;
    let skill_context = ctx(_rt);

    match skill_context.setting(&*name) {
        Some(v) => {
            Ok(PushVariable::push_var(&v.value.clone()))
        }
        None => {
            Ok(PushVariable::push_var(&YamlValue(Yaml::Null)))
        }
    }
}

#[allow(non_snakeCase)]
pub fn get_setting_full(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt);

    let val = ctx.settings.iter()
        .find(|s| s.name == name)
        .cloned()
        .unwrap_or(SettingNamed {
            name,
            setting: Setting::default()
        });

    Ok(PushVariable::push_var(&val))
}

#[allow(non_snake_case)]
pub fn list_settings(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt);

    let list: Vec<(String, YamlValue)> =
        ctx.settings.iter()
            .map(|s| (s.name.clone(), s.setting.value.clone()))
            .collect();

    Ok(PushVariable::push_var(&list))
}

#[allow(non_snake_case)]
pub fn has_setting(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt);

    let exists = ctx.settings.iter().any(|s| s.name == name);
    Ok(PushVariable::push_var(&exists))
}
