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

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageFile {
    pub code: String,
    pub lang: HashMap<String, YamlValue>,
}

#[derive(Debug, Clone, Deserialize)]
pub  struct IndividualLocale {
    pub id: String,
    pub value: YamlValue,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    pub code: String,
    pub lang: Vec<IndividualLocale>,
}

#[derive(Debug, Clone)]
pub struct SkillContext {
    pub(crate) constants: Vec<ConstantNamed>,
    pub(crate) settings: Vec<SettingNamed>,
    pub(crate) languages: Vec<Language>,
}


impl SkillContext {
    pub fn from_yaml(path: &str) -> anyhow::Result<Self> {
        let content_const = fs::read_to_string(format!("{}/config/const.yaml", path))?;
        let parsed_const: ConstFile = serde_yaml::from_str(&content_const)?;

        let content_settings = fs::read_to_string(format!("{}/config/settings.yaml", path))?;
        let parsed_settings: SettingsFile = serde_yaml::from_str(&content_settings)?;
        
        // Load language response files from the `responses` directory (e.g., en.yaml, pt.yaml)
        let languages = Self::load_languages(path)?;
        
        Ok(Self {
            constants: Self::const_to_named(&parsed_const.constants),
            settings: Self::settings_to_named(&parsed_settings.settings),
            languages,
        })
    }

    fn load_languages(path: &str) -> anyhow::Result<Vec<Language>> {
        let mut languages: Vec<Language> = Vec::new();
        let responses_dir = format!("{}/responses", path);

        // If responses directory does not exist, just return empty languages
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
                if ext == "yaml" || ext == "yml" {
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

    pub fn locale(&self, code: &str, id: &str) -> Option<YamlValue> {
        use rand::seq::SliceRandom;

        self.languages
            .iter()
            .find(|l| l.code == code)
            .and_then(|l| l.lang.iter().find(|i| i.id == id))
            .map(|i| {
                match &i.value.0 {
                    serde_yaml::Value::Sequence(seq) if !seq.is_empty() => {
                        let mut rng = rand::thread_rng();
                        seq.choose(&mut rng)
                            .map(|v| YamlValue(v.clone()))
                            .unwrap_or_else(|| i.value.clone())
                    }
                    _ => i.value.clone()
                }
            })
    }
}
