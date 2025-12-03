use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use crate::intent::YamlValue;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize)]
pub struct ConstFile {
    pub constants: HashMap<String, serde_yaml::Value>,
}


#[derive(Debug, Clone)]
pub struct SkillConfig {
    constants: HashMap<String, serde_yaml::Value>,
    map: HashMap<String, Setting>,
}
impl SkillConfig {
    pub fn from_yaml(path: &str) -> anyhow::Result<Self> {
        let content_const = fs::read_to_string(format!("{}/config/const.yaml", path))?;
        let parsed_const: ConstFile = serde_yaml::from_str(&content_const)?;

        let content_settings = fs::read_to_string(format!("{}/config/settings.yaml", path))?;
        let parsed_settings: SettingsFile = serde_yaml::from_str(&content_settings)?;

        Ok(Self {
            constants: parsed_const.constants,
            map: parsed_settings.settings,
        })
    }

    pub fn get(&self, name: &str) -> Option<&Setting> {
        self.map.get(name)
    }

    pub fn constant(&self, name: &str) -> Option<&serde_yaml::Value> {
        self.constants.get(name)
    }

    pub fn get_typed<T: serde::de::DeserializeOwned>(&self, name: &str) -> Option<T> {
        self.map.get(name).and_then(|s| {
            serde_yaml::from_value(s.value.clone()).ok()
        })
    }
}
