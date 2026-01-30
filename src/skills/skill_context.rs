use crate::data::config::ConfigSystem;
use crate::dialogue::languages::LanguageSystem;
use crate::utils::load_value_from_file;
use memory_size_derive::{DeepSize, DeepSizeTree};
use rhai::CustomType;
use rhai::TypeBuilder;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Helper function to provide a default value of `true` for serde.
fn default_true() -> bool {
    true
}

/// The manifest file containing metadata and configuration for a skill.
#[derive(Debug, Serialize, Default, Deserialize, Clone, CustomType, DeepSize, DeepSizeTree)]
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
    /// A list of subscriptions the skill asks.
    pub subscription: Vec<String>,
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
#[derive(Debug, Clone, CustomType, DeepSize, DeepSizeTree)]
pub struct SkillContext {
    #[rhai_type(readonly)]
    /// The filesystem path to the skill directory.
    pub path: Arc<str>,
    #[rhai_type(readonly)]
    /// Metadata about the skill.
    pub info: Arc<Manifest>,

    #[rhai_type(skip)]
    #[deep_size(opaque)]
    pub config: Arc<ConfigSystem>,

    #[rhai_type(skip)]
    #[deep_size(opaque)]
    /// Localized resources for the skill.
    pub languages: Arc<LanguageSystem>,
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
            path: Arc::from(path),
            info: Arc::new(load_value_from_file(
                format!("{}/manifest.yaml", path).into(),
            )?),
            config: Arc::new(ConfigSystem::new(&format!("{}/config", path))),
            languages: Arc::new(LanguageSystem::new(&format!("{}/responses", path))),
        })
    }
}
