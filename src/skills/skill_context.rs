use crate::config::ConfigSystem;
use crate::dialogue::languages::LanguageSystem;
use serde::{Deserialize, Serialize};
use std::fs;

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

    pub config: ConfigSystem,

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
    pub fn new(path: &str) -> Result<Self, String> {
        Ok(Self {
            path: path.to_string(),
            info: Self::load_manifest(path.into())?,
            config: ConfigSystem::new(&format!("{}/config", path)),
            languages: LanguageSystem::new(&format!("{}/responses", path)),
        })
    }

    /// Loads the skill manifest from the filesystem.
    ///
    /// # Panics
    ///
    /// Panics if the manifest file cannot be read or parsed.
    fn load_manifest(pathname: String) -> Result<Manifest, String> {
        let manifest_path = format!("{}/manifest.yaml", pathname);
        let manifest_file = match fs::read_to_string(manifest_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error reading manifest file: {}", e)),
        };
        match serde_yaml::from_str(&manifest_file) {
            Ok(manifest) => Ok(manifest),
            Err(e) => Err(format!("Error parsing manifest file: {}", e)),
        }
    }

    /// Serializes the `SkillContext` into a JSON string.
    #[allow(dead_code)]
    pub fn into_json(self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}
