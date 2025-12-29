use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use rand::seq::IndexedRandom;
use crate::intent::YamlValue;

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

/// Represents the structure of a language resource file.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageFile {
    /// The language code (e.g., "en", "pt").
    pub code: String,
    /// A map of resource IDs to their localized values.
    pub lang: HashMap<String, YamlValue>,
}

/// A localized resource entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub  struct IndividualLocale {
    /// The unique identifier for the localized resource.
    pub id: String,
    /// The localized value (can be a string or a list of strings for randomization).
    pub value: YamlValue,
}

/// Represents a collection of localized resources for a specific language.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Language {
    /// The language code.
    pub code: String,
    /// The list of localized resources.
    pub lang: Vec<IndividualLocale>,
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
    pub(crate) languages: Vec<Language>,
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
        let languages = Self::load_languages(path)?;
        
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

    /// Scans the `responses` directory and loads all available language resource files.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The skill's root directory path.
    fn load_languages(path: &str) -> anyhow::Result<Vec<Language>> {
        let mut languages: Vec<Language> = Vec::new();
        let responses_dir = format!("{}/responses", path);

        let read_dir = match fs::read_dir(&responses_dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(languages),
        };

        for entry in read_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path_buf = entry.path();
            if let Some(ext) = path_buf.extension() {
                if ext == "yaml" || ext == "lang" {
                    if let Ok(content) = fs::read_to_string(&path_buf) {
                        if let Ok(parsed) = serde_yaml::from_str::<LanguageFile>(&content) {
                            let lang_vec: Vec<IndividualLocale> = parsed
                                .lang
                                .into_iter()
                                .map(|(id, value)| IndividualLocale { id, value })
                                .collect();
                            languages.push(Language { code: parsed.code, lang: lang_vec });
                        }
                    }
                }
            }
        }

        Ok(languages)
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
        self.languages
            .iter()
            .find(|l| l.code == code)
            .and_then(|l| l.lang.iter().find(|i| i.id == id))
            .map(|i| {
                match &i.value.0 {
                    serde_yaml::Value::Sequence(seq) if !seq.is_empty() => {
                        let mut rng = rand::rng();
                        seq.choose(&mut rng)
                            .map(|v| YamlValue(v.clone()))
                            .unwrap_or_else(|| i.value.clone())
                    }
                    _ => i.value.clone()
                }
            })
    }

    /// Serializes the `SkillContext` into a JSON string.
    #[allow(dead_code)]
    pub fn into_json(self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}
