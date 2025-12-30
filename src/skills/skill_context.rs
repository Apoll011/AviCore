use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use rand::seq::IndexedRandom;
use crate::intent::YamlValue;
use crate::languages::LanguageSystem;

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
pub  struct ConstantNamed {
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

/// Helper function to provide a default value of `true` for serde.
fn default_true() -> bool {
    true
}

/// The manifest file containing metadata and configuration for a skill.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    /// Unique identifier for the skill.
    pub id: String,
    /// Human-readable name of the skill.
    pub name: String,
    /// A brief description of what the skill does.
    pub description: String,
    /// Indicates if the skill is currently disabled.
    #[serde(default)]
    pub disabled: bool,
    /// The entry point filename for the skill's logic (e.g., "main.avi").
    pub entry: String,
    /// A list of capabilities required by the skill.
    pub capabilities: Vec<String>,
    /// A list of permissions required by the skill.
    pub permissions: Vec<String>,
    /// Whether the skill supports repeating the last response.
    #[serde(default = "default_true")]
    pub can_repeat_last_response: bool,
    /// Whether the skill supports immediate re-execution.
    #[serde(default = "default_true")]
    pub can_go_again: bool,
    /// The author of the skill.
    pub author: String,
    /// The version of the skill.
    pub version: String,
}

/// The complete context of a skill, including its manifest, constants, settings, and localized resources.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SkillContext {
    /// The filesystem path to the skill directory.
    pub path: String,
    /// Metadata about the skill.
    pub(crate) info: Manifest,
    /// Constants defined for the skill.
    pub(crate) constants: Vec<ConstantNamed>,
    /// Settings defined for the skill.
    pub(crate) settings: Vec<SettingNamed>,
    /// Localized resources for the skill.
    pub(crate) languages: LanguageSystem,
}

impl SkillContext {
    /// Initializes a `SkillContext` by loading configuration files from the specified path.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The directory path containing the skill's configuration.
    /// 
    /// # Errors
    /// 
    /// Returns an error if any of the configuration files are missing or malformed.
    /// 
    /// FIXME: Using `anyhow::Result` here while other parts of the codebase use `Box<dyn std::error::Error>`. Consistency would be better.
    pub fn from_yaml(path: &str) -> anyhow::Result<Self> {
        let content_const = fs::read_to_string(format!("{}/config/const.config", path))?;
        let parsed_const: ConstFile = serde_yaml::from_str(&content_const)?;

        let content_settings = fs::read_to_string(format!("{}/config/settings.config", path))?;
        let parsed_settings: SettingsFile = serde_yaml::from_str(&content_settings)?;
        
        // Load language response files from the `responses` directory (e.g., en.yaml, pt.yaml)
        let languages = LanguageSystem::new(&format!("{}/responses", path));
        
        Ok(Self {
            path: path.to_string(),
            info:  Self::load_manifest(path.into()),
            constants: Self::const_to_named(&parsed_const.constants),
            settings: Self::settings_to_named(&parsed_settings.settings),
            languages,
        })
    }

    /// Loads the skill manifest from the filesystem.
    /// 
    /// # Panics
    /// 
    /// Panics if the manifest file cannot be read or parsed.
    /// 
    /// TODO: Handle manifest loading errors gracefully instead of panicking.
    fn load_manifest(pathname: String) -> Manifest {
        let manifest_path = format!("{}/manifest.yaml", pathname);
        let manifest_file = fs::read_to_string(manifest_path).expect("Could not read manifest file");
        serde_yaml::from_str(&manifest_file).expect("Could not parse manifest file")
    }

    /// Converts a map of constants to a vector of `ConstantNamed`.
    fn const_to_named(constants: &HashMap<String, YamlValue>) -> Vec<ConstantNamed> {
        constants.iter().map(|(k, v)| ConstantNamed { name: k.clone(), value: v.clone() }).collect()
    }

    /// Converts a map of settings to a vector of `SettingNamed`.
    fn settings_to_named(settings: &HashMap<String, Setting>) -> Vec<SettingNamed> {
        settings.iter().map(|(k, v)| SettingNamed { name: k.clone(), setting: v.clone() }).collect()
    }

    /// Retrieves a setting by its name.
    pub fn setting(&self, name: &str) -> Option<&Setting> {
        self.settings.iter().find(|s| s.name == name).map(|s| &s.setting)
    }

    /// Retrieves a constant value by its name.
    pub fn constant(&self, name: &str) -> Option<&YamlValue> {
        self.constants.iter().find(|c| c.name == name).map(|c| &c.value)
    }

    /// Retrieves a localized resource value.
    /// 
    /// If the value is a list (sequence), it randomly selects one entry from the list.
    /// 
    /// # Arguments
    /// 
    /// * `code` - The language code.
    /// * `id` - The resource identifier.
    pub fn locale(&self, code: &str, id: &str) -> Option<YamlValue> {
        self.languages.locale(code, id)
    }

    /// Serializes the `SkillContext` into a JSON string.
    #[allow(dead_code)]
    pub fn into_json(self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}
