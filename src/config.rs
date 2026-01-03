use crate::dialogue::intent::YamlValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
/// Represents a specific configuration setting for a skill.
pub struct Setting {
    /// The current value of the setting.
    pub value: YamlValue,
    /// The data type of the setting (e.g., "string", "int").
    #[serde(default)]
    pub vtype: Option<String>,
    /// A human-readable description of the setting.
    #[serde(default)]
    pub description: Option<String>,
    /// UI-related configuration for the setting.
    #[serde(default)]
    pub ui: Option<String>,
    /// Indicates if the setting is required.
    #[serde(default)]
    pub required: Option<bool>,
    /// Minimum value or length (optional).
    #[serde(default)]
    pub min: Option<usize>,
    /// Maximum value or length (optional).
    #[serde(default)]
    pub max: Option<usize>,
    /// A list of allowed values if the setting is an enum.
    #[serde(default)]
    pub enum_: Option<Vec<String>>,
    /// Indicates if this is an advanced setting.
    #[serde(default)]
    pub advanced: Option<bool>,
    /// The group name for organizing settings in a UI.
    #[serde(default)]
    pub group: Option<String>,
}

/// Represents the structure of a settings configuration file.
#[derive(Debug, Deserialize)]
pub struct SettingsFile {
    /// A map of setting names to their definitions.
    pub settings: HashMap<String, Setting>,
}

/// Represents the structure of a constants configuration file.
#[derive(Deserialize)]
pub struct ConstFile {
    /// A map of constant names to their values.
    pub constants: HashMap<String, YamlValue>,
}

/// A named constant value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConstantNamed {
    /// The name of the constant.
    pub name: String,
    /// The value of the constant.
    pub value: YamlValue,
}

/// A named setting definition.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingNamed {
    /// The name of the setting.
    pub name: String,
    /// The definition of the setting.
    pub setting: Setting,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConfigSystem {
    /// Constants defined for the skill.
    pub(crate) constants: Vec<ConstantNamed>,
    /// Settings defined for the skill.
    pub(crate) settings: Vec<SettingNamed>,
}

impl ConfigSystem {
    pub fn new(path: &str) -> Self {
        let Ok(content) = fs::read_to_string(format!("{}/const.config", path)) else {
            return ConfigSystem::default();
        };

        let Ok(parsed_const) = serde_yaml::from_str::<ConstFile>(&content) else {
            return ConfigSystem::default();
        };

        let Ok(content_settings) = fs::read_to_string(format!("{}/settings.config", path)) else {
            return ConfigSystem::default();
        };

        let Ok(parsed_settings) = serde_yaml::from_str::<SettingsFile>(&content_settings) else {
            return ConfigSystem::default();
        };

        Self {
            constants: Self::const_to_named(&parsed_const.constants),
            settings: Self::settings_to_named(&parsed_settings.settings),
        }
    }

    /// Converts a map of constants to a vector of `ConstantNamed`.
    fn const_to_named(constants: &HashMap<String, YamlValue>) -> Vec<ConstantNamed> {
        constants
            .iter()
            .map(|(k, v)| ConstantNamed {
                name: k.clone(),
                value: v.clone(),
            })
            .collect()
    }

    /// Converts a map of settings to a vector of `SettingNamed`.
    fn settings_to_named(settings: &HashMap<String, Setting>) -> Vec<SettingNamed> {
        settings
            .iter()
            .map(|(k, v)| SettingNamed {
                name: k.clone(),
                setting: v.clone(),
            })
            .collect()
    }

    /// Retrieves a setting by its name.
    pub fn setting(&self, name: &str) -> Option<&Setting> {
        self.settings
            .iter()
            .find(|s| s.name == name)
            .map(|s| &s.setting)
    }

    /// Retrieves a constant value by its name.
    pub fn constant(&self, name: &str) -> Option<&YamlValue> {
        self.constants
            .iter()
            .find(|c| c.name == name)
            .map(|c| &c.value)
    }

    pub fn list_constants(&self) -> Vec<(String, YamlValue)> {
        self.constants
            .iter()
            .map(|c| (c.name.clone(), c.value.clone()))
            .collect()
    }

    pub fn has_constant(&self, name: &str) -> bool {
        self.constants.iter().any(|c| c.name == name)
    }

    pub fn list_settings(&self) -> Vec<(String, YamlValue)> {
        self.settings
            .iter()
            .map(|s| (s.name.clone(), s.setting.value.clone()))
            .collect()
    }

    pub fn has_setting(&self, name: &str) -> bool {
        self.settings.iter().any(|s| s.name == name)
    }

    pub fn get_setting_full(&self, name: &str) -> SettingNamed {
        self.settings
            .iter()
            .find(|s| s.name == name)
            .cloned()
            .unwrap_or(SettingNamed {
                name: name.to_string(),
                setting: Setting::default(),
            })
    }
}
