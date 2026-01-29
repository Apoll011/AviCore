use crate::data::config::setting_or;
use crate::ctx::runtime;
use log::{debug, error, info, trace, warn};
use rand::prelude::IndexedRandom;
use rhai::CustomType;
use rhai::TypeBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use strfmt::strfmt;

/// Represents the structure of a language resource file.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageFile {
    /// The language code (e.g., "en", "pt").
    pub code: String,
    /// A map of resource IDs to their localized values.
    pub lang: HashMap<String, serde_yaml::Value>,
}

/// A localized resource entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndividualLocale {
    /// The unique identifier for the localized resource.
    pub id: String,
    /// The localized value (can be a string or a list of strings for randomization).
    pub value: serde_yaml::Value,
}

/// Represents a collection of localized resources for a specific language.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Language {
    /// The language code.
    pub code: String,
    /// The list of localized resources.
    pub lang: Vec<IndividualLocale>,
}

#[derive(Debug, Clone, Deserialize, Serialize, CustomType)]
pub struct LanguageSystem {
    pub languages: Vec<Language>,
}

impl LanguageSystem {
    /// Scans the `responses` directory and loads all available language resource files.
    ///
    /// # Arguments
    ///
    /// * `path` - The skill's root directory path.
    pub fn new(path: &str) -> Self {
        trace!("Initializing LanguageSystem from {}", path);
        let mut languages: Vec<Language> = Vec::new();

        let read_dir = match fs::read_dir(path) {
            Ok(rd) => rd,
            Err(e) => {
                warn!("Failed to read language directory {}: {}", path, e);
                return Self { languages };
            }
        };

        for entry in read_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    warn!("Failed to read directory entry in {}: {}", path, e);
                    continue;
                }
            };
            let path_buf = entry.path();
            if let Some(ext) = path_buf.extension()
                && (ext == "yaml" || ext == "lang")
            {
                match fs::read_to_string(&path_buf) {
                    Ok(content) => match serde_yaml::from_str::<LanguageFile>(&content) {
                        Ok(parsed) => {
                            let lang_vec: Vec<IndividualLocale> = parsed
                                .lang
                                .into_iter()
                                .map(|(id, value)| IndividualLocale { id, value })
                                .collect();
                            debug!(
                                "Loaded language {} from {}",
                                parsed.code,
                                path_buf.display()
                            );
                            languages.push(Language {
                                code: parsed.code,
                                lang: lang_vec,
                            });
                        }
                        Err(e) => error!(
                            "Failed to parse language file {}: {}",
                            path_buf.display(),
                            e
                        ),
                    },
                    Err(e) => error!("Failed to read language file {}: {}", path_buf.display(), e),
                }
            }
        }
        info!("Loaded {} languages from {}", languages.len(), path);
        Self { languages }
    }

    /// Retrieves a localized resource value.
    ///
    /// If the value is a list (sequence), it randomly selects one entry from the list.
    ///
    /// # Arguments
    ///
    /// * `code` - The language code.
    /// * `id` - The resource identifier.
    pub fn locale(&self, code: &str, id: &str) -> Option<serde_yaml::Value> {
        self.languages
            .iter()
            .find(|l| l.code == code)
            .and_then(|l| l.lang.iter().find(|i| i.id == id))
            .map(|i| match &i.value {
                serde_yaml::Value::Sequence(seq) if !seq.is_empty() => {
                    let mut rng = rand::rng();
                    seq.choose(&mut rng)
                        .cloned()
                        .unwrap_or_else(|| i.value.clone())
                }
                _ => i.value.clone(),
            })
    }

    pub fn locale_fmt(
        &self,
        code: &str,
        id: &str,
        fmt: &HashMap<String, String>,
    ) -> Option<String> {
        match self.locale(code, id) {
            Some(value) => self
                .value_to_string(&value)
                .map(|v| strfmt(&v, fmt).unwrap_or_else(|_| v.clone())),
            None => None,
        }
    }

    fn value_to_string(&self, value: &serde_yaml::Value) -> Option<String> {
        match &value {
            serde_yaml::Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_translation(&self, id: &str) -> Option<String> {
        match self.locale(&lang(), id) {
            Some(value) => self.value_to_string(&value),
            None => None,
        }
    }

    pub fn get_translation_list(&self, id: &str) -> Vec<String> {
        self.languages
            .iter()
            .find(|l| l.code == *lang())
            .and_then(|l| l.lang.iter().find(|i| i.id == id))
            .map(|i| match &i.value {
                serde_yaml::Value::Sequence(seq) if !seq.is_empty() => seq
                    .iter()
                    .filter_map(|v| {
                        if let serde_yaml::Value::String(s) = v {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
                serde_yaml::Value::String(s) => vec![s.clone()],
                _ => Vec::new(),
            })
            .unwrap_or_else(Vec::new)
    }

    pub fn list(&self, code: &str) -> HashMap<String, serde_yaml::Value> {
        let Some(lang) = self.languages.iter().find(|l| l.code == code) else {
            return HashMap::<String, serde_yaml::Value>::new();
        };

        lang.lang
            .iter()
            .map(|i| (i.id.clone(), i.value.clone()))
            .collect()
    }

    pub fn has(&self, id: &str) -> bool {
        self.languages
            .iter()
            .any(|lang| lang.lang.iter().any(|l| l.id == id))
    }
}

pub fn locale(key: &str) -> Option<String> {
    match runtime() {
        Ok(c) => c.language_system.get_translation(key),
        Err(e) => {
            debug!("Failed to get translation for {}: {}", key, e);
            None
        }
    }
}

pub fn get_translation_list(key: &str) -> Vec<String> {
    match runtime() {
        Ok(c) => c.language_system.get_translation_list(key),
        Err(e) => {
            debug!("Failed to get translation list for {}: {}", key, e);
            Vec::new()
        }
    }
}

pub fn lang() -> String {
    setting_or::<String>("lang", "en".to_string())
}
