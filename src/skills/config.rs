use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use crate::intent::YamlValue;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Setting {
    pub value: YamlValue,
    #[serde(default)]
    pub vtype: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub ui: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub min: Option<usize>,
    #[serde(default)]
    pub max: Option<usize>,
    #[serde(default)]
    pub enum_: Option<Vec<String>>,
    #[serde(default)]
    pub advanced: Option<bool>,
    #[serde(default)]
    pub group: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SettingsFile {
    pub settings: HashMap<String, Setting>,
}

#[derive(Deserialize)]
pub struct ConstFile {
    pub constants: HashMap<String, YamlValue>,
}

#[derive(Debug, Clone)]
pub  struct ConstantNamed {
    pub name: String,
    pub value: YamlValue,
}

#[derive(Debug, Clone)]
pub struct SettingNamed {
    pub name: String,
    pub setting: Setting,
}

#[derive(Debug, Clone)]
pub struct SkillConfig {
    pub(crate) constants: Vec<ConstantNamed>,
    pub(crate) settings: Vec<SettingNamed>,
}

impl SkillConfig {
    pub fn from_yaml(path: &str) -> anyhow::Result<Self> {
        let content_const = fs::read_to_string(format!("{}/config/const.yaml", path))?;
        let parsed_const: ConstFile = serde_yaml::from_str(&content_const)?;

        let content_settings = fs::read_to_string(format!("{}/config/settings.yaml", path))?;
        let parsed_settings: SettingsFile = serde_yaml::from_str(&content_settings)?;

        Ok(Self {
            constants: Self::const_to_named(&parsed_const.constants),
            settings: Self::settings_to_named(&parsed_settings.settings),
        })
    }

    fn const_to_named(constants: &HashMap<String, YamlValue>) -> Vec<ConstantNamed> {
        constants.iter().map(|(k, v)| ConstantNamed { name: k.clone(), value: v.clone() }).collect()
    }

    fn settings_to_named(settings: &HashMap<String, Setting>) -> Vec<SettingNamed> {
        settings.iter().map(|(k, v)| SettingNamed { name: k.clone(), setting: v.clone() }).collect()
    }

    pub fn setting(&self, name: &str) -> Option<&Setting> {
        self.settings.iter().find(|s| s.name == name).map(|s| &s.setting)
    }

    pub fn constant(&self, name: &str) -> Option<&YamlValue> {
        self.constants.iter().find(|c| c.name == name).map(|c| &c.value)
    }
}
