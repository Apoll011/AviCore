use crate::dialogue::languages::get_translation_list;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Position;
use rhai::TypeBuilder;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    #[allow(dead_code)]
    ParseError(String),
    NotAccepted,
}

pub trait ResponseValidator {
    type Output;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError>;

    #[allow(dead_code)]
    fn is_accepted(&self, text: &str) -> bool {
        self.validate_and_parse(text).is_ok()
    }

    fn clear_text(&self, text: &str) -> String {
        text.trim().to_string()
    }

    fn get_error_txt(&self, error: &ValidationError) -> String;
}

#[derive(Debug, Deserialize, CustomType, Clone)]
pub struct AnyValidator;

impl AnyValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ResponseValidator for AnyValidator {
    type Output = String;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        Ok(self.clear_text(text))
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        "error_any".to_string()
    }
}

#[derive(Debug, Deserialize, CustomType, Clone)]
pub struct ListOrNoneValidator {
    pub allowed_values: Vec<String>,
}

impl ListOrNoneValidator {
    pub fn new(allowed_values: Vec<String>) -> Self {
        Self { allowed_values }
    }
}

impl ResponseValidator for ListOrNoneValidator {
    type Output = Option<String>;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();

        let none_translations = get_translation_list("none");

        for none_text in none_translations {
            if cleaned.contains(&none_text.to_lowercase()) {
                return Ok(None);
            }
        }

        for allowed in &self.allowed_values {
            let compare_allowed = allowed.to_lowercase();

            if cleaned.contains(&compare_allowed) {
                return Ok(Some(cleaned));
            }
        }

        Err(ValidationError::NotAccepted)
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        "not_valid_error".to_string()
    }
}

#[derive(Debug, Deserialize, CustomType, Clone)]
pub struct OptionalValidator;
impl OptionalValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ResponseValidator for OptionalValidator {
    type Output = Option<String>;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text);
        let cleaned_lower = cleaned.to_lowercase();

        let none_translations = get_translation_list("none");

        for none_text in none_translations {
            if cleaned_lower.contains(&none_text.to_lowercase()) {
                return Ok(None);
            }
        }

        Ok(Some(cleaned))
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        "error_validator_optional".to_string()
    }
}

#[derive(Debug, Deserialize, CustomType, Clone)]
pub struct BoolValidator {
    pub hard_search: bool,
}

impl BoolValidator {
    pub fn new(hard_search: bool) -> Self {
        Self { hard_search }
    }
}

impl ResponseValidator for BoolValidator {
    type Output = bool;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();

        let yes_translations = get_translation_list("yes");
        let no_translations = get_translation_list("no");

        if self.hard_search {
            for yes_text in &yes_translations {
                if cleaned == yes_text.to_lowercase() {
                    return Ok(true);
                }
            }
            for no_text in &no_translations {
                if cleaned == no_text.to_lowercase() {
                    return Ok(false);
                }
            }
        } else {
            for yes_text in &yes_translations {
                if cleaned.contains(&yes_text.to_lowercase()) {
                    return Ok(true);
                }
            }
            for no_text in &no_translations {
                if cleaned.contains(&no_text.to_lowercase()) {
                    return Ok(false);
                }
            }
        }

        Err(ValidationError::NotAccepted)
    }
    fn get_error_txt(&self, _error: &ValidationError) -> String {
        "error_validator_bool".to_string()
    }
}

#[derive(Clone, CustomType)]
pub struct MappedValidator {
    pub mappings: HashMap<String, Dynamic>,
    pub default: Option<Dynamic>,
    pub hard_search: bool,
}

impl MappedValidator {
    pub fn new(mappings: HashMap<String, Dynamic>) -> Self {
        Self {
            mappings,
            default: None,
            hard_search: false,
        }
    }

    pub fn with_default(mut self, default: Dynamic) -> Self {
        self.default = Some(default);
        self
    }

    #[allow(dead_code)]
    pub fn hard_search(mut self, enabled: bool) -> Self {
        self.hard_search = enabled;
        self
    }
}

impl ResponseValidator for MappedValidator {
    type Output = Dynamic;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();

        let result = if self.hard_search {
            self.mappings.get(&cleaned).cloned()
        } else {
            let mut found = None;
            for (key, value) in &self.mappings {
                if cleaned.contains(&key.to_lowercase()) {
                    found = Some(value.clone());
                    break;
                }
            }
            found
        };

        match result {
            Some(value) => Ok(value),
            None => self.default.clone().ok_or(ValidationError::NotAccepted),
        }
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        "not_valid_error".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_validator_basic() {
        let validator = AnyValidator;
        assert_eq!(validator.validate_and_parse("hello").unwrap(), "hello");
        assert_eq!(
            validator.validate_and_parse("  spaces  ").unwrap(),
            "spaces"
        );
        assert!(validator.is_accepted("anything works"));
    }

    #[test]
    fn test_any_validator_edge_cases() {
        let validator = AnyValidator;
        assert_eq!(validator.validate_and_parse("").unwrap(), "");
        assert_eq!(validator.validate_and_parse("   ").unwrap(), "");
        assert_eq!(validator.validate_and_parse("123!@#").unwrap(), "123!@#");
    }

    #[test]
    fn test_list_or_none_validator_with_none() {
        let validator = ListOrNoneValidator::new(vec!["apple".to_string(), "banana".to_string()]);

        assert_eq!(validator.validate_and_parse("none").unwrap(), None);
        assert_eq!(validator.validate_and_parse("NONE").unwrap(), None);
        assert_eq!(validator.validate_and_parse("I want none").unwrap(), None);
    }

    #[test]
    fn test_list_or_none_validator_with_values() {
        let validator = ListOrNoneValidator::new(vec!["apple".to_string(), "banana".to_string()]);

        assert_eq!(
            validator.validate_and_parse("apple").unwrap(),
            Some("apple".to_string())
        );
        assert_eq!(
            validator.validate_and_parse("I like banana").unwrap(),
            Some("I like banana".to_string())
        );
        assert_eq!(
            validator.validate_and_parse("APPLE").unwrap(),
            Some("APPLE".to_string())
        );
    }

    #[test]
    fn test_list_or_none_validator_case_sensitive() {
        let validator = ListOrNoneValidator::new(vec!["Apple".to_string()]);

        assert!(validator.validate_and_parse("apple").is_err());
        assert_eq!(
            validator.validate_and_parse("Apple").unwrap(),
            Some("Apple".to_string())
        );
        assert!(validator.validate_and_parse("none").is_err());
        assert_eq!(validator.validate_and_parse("None").unwrap(), None);
    }

    #[test]
    fn test_list_or_none_validator_not_accepted() {
        let validator = ListOrNoneValidator::new(vec!["apple".to_string()]);

        assert!(validator.validate_and_parse("orange").is_err());
        assert!(validator.validate_and_parse("random text").is_err());
    }

    #[test]
    fn test_optional_validator_with_none() {
        let validator = OptionalValidator;

        assert_eq!(validator.validate_and_parse("none").unwrap(), None);
        assert_eq!(validator.validate_and_parse("NONE").unwrap(), None);
        assert_eq!(validator.validate_and_parse("I choose none").unwrap(), None);
    }

    #[test]
    fn test_optional_validator_with_value() {
        let validator = OptionalValidator;

        assert_eq!(
            validator.validate_and_parse("something").unwrap(),
            Some("something".to_string())
        );
        assert_eq!(
            validator.validate_and_parse("  value  ").unwrap(),
            Some("value".to_string())
        );
    }

    #[test]
    fn test_bool_validator_basic() {
        let validator = BoolValidator::new(false);

        assert_eq!(validator.validate_and_parse("yes").unwrap(), true);
        assert_eq!(validator.validate_and_parse("no").unwrap(), false);
        assert_eq!(validator.validate_and_parse("always").unwrap(), true);
        assert_eq!(validator.validate_and_parse("never").unwrap(), false);
    }

    #[test]
    fn test_bool_validator_case_insensitive() {
        let validator = BoolValidator::new(false);

        assert_eq!(validator.validate_and_parse("YES").unwrap(), true);
        assert_eq!(validator.validate_and_parse("No").unwrap(), false);
        assert_eq!(validator.validate_and_parse("ALWAYS").unwrap(), true);
        assert_eq!(validator.validate_and_parse("NeVeR").unwrap(), false);
    }

    #[test]
    fn test_bool_validator_partial_match() {
        let validator = BoolValidator::new(false);

        assert_eq!(validator.validate_and_parse("oh yes please").unwrap(), true);
        assert_eq!(validator.validate_and_parse("no way").unwrap(), false);
        assert_eq!(
            validator.validate_and_parse("I will always do it").unwrap(),
            true
        );
    }

    #[test]
    fn test_bool_validator_hard_search() {
        let validator = BoolValidator::new(true);

        assert_eq!(validator.validate_and_parse("yes").unwrap(), true);
        assert!(validator.validate_and_parse("oh yes please").is_err());
        assert!(validator.validate_and_parse("maybe").is_err());
    }

    #[test]
    fn test_bool_validator_not_accepted() {
        let validator = BoolValidator::new(false);

        assert!(validator.validate_and_parse("maybe").is_err());
        assert!(validator.validate_and_parse("sometimes").is_err());
        assert!(validator.validate_and_parse("").is_err());
    }

    #[test]
    fn test_mapped_validator_basic() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), Dynamic::from(1i32));
        mappings.insert("blue".to_string(), Dynamic::from(2i32));
        mappings.insert("green".to_string(), Dynamic::from(3i32));

        let validator = MappedValidator::new(mappings);

        assert_eq!(
            validator.validate_and_parse("red").unwrap().cast::<i32>(),
            1
        );
        assert_eq!(
            validator.validate_and_parse("blue").unwrap().cast::<i32>(),
            2
        );
        assert_eq!(
            validator.validate_and_parse("green").unwrap().cast::<i32>(),
            3
        );
    }

    #[test]
    fn test_mapped_validator_with_default() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), Dynamic::from(1i32));
        mappings.insert("blue".to_string(), Dynamic::from(2i32));

        let validator = MappedValidator::new(mappings).with_default(Dynamic::from(0i32));

        assert_eq!(
            validator.validate_and_parse("red").unwrap().cast::<i32>(),
            1
        );
        assert_eq!(
            validator
                .validate_and_parse("unknown")
                .unwrap()
                .cast::<i32>(),
            0
        );
        assert_eq!(
            validator.validate_and_parse("xyz").unwrap().cast::<i32>(),
            0
        );
    }

    #[test]
    fn test_mapped_validator_without_default() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), Dynamic::from(1i32));

        let validator = MappedValidator::new(mappings);

        assert_eq!(
            validator.validate_and_parse("red").unwrap().cast::<i32>(),
            1
        );
        assert!(validator.validate_and_parse("unknown").is_err());
    }

    #[test]
    fn test_mapped_validator_partial_match() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), Dynamic::from(1i32));
        mappings.insert("blue".to_string(), Dynamic::from(2i32));

        let validator = MappedValidator::new(mappings);

        assert_eq!(
            validator
                .validate_and_parse("I like red")
                .unwrap()
                .cast::<i32>(),
            1
        );
        assert_eq!(
            validator
                .validate_and_parse("dark blue color")
                .unwrap()
                .cast::<i32>(),
            2
        );
    }

    #[test]
    fn test_mapped_validator_hard_search() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), Dynamic::from(1i32));
        mappings.insert("blue".to_string(), Dynamic::from(2i32));

        let validator = MappedValidator::new(mappings).hard_search(true);

        assert_eq!(
            validator.validate_and_parse("red").unwrap().cast::<i32>(),
            1
        );
        assert!(validator.validate_and_parse("I like red").is_err());
        assert!(validator.validate_and_parse("dark blue").is_err());
    }

    #[test]
    fn test_mapped_validator_case_insensitive() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), Dynamic::from(1i32));

        let validator = MappedValidator::new(mappings);

        assert_eq!(
            validator.validate_and_parse("RED").unwrap().cast::<i32>(),
            1
        );
        assert_eq!(
            validator.validate_and_parse("Red").unwrap().cast::<i32>(),
            1
        );
    }

    #[test]
    fn test_mapped_validator_with_strings() {
        let mut mappings = HashMap::new();
        mappings.insert("small".to_string(), Dynamic::from("S".to_string()));
        mappings.insert("medium".to_string(), Dynamic::from("M".to_string()));
        mappings.insert("large".to_string(), Dynamic::from("L".to_string()));

        let validator = MappedValidator::new(mappings).with_default(Dynamic::from("?".to_string()));

        assert_eq!(
            validator
                .validate_and_parse("small")
                .unwrap()
                .cast::<String>(),
            "S"
        );
        assert_eq!(
            validator
                .validate_and_parse("medium")
                .unwrap()
                .cast::<String>(),
            "M"
        );
        assert_eq!(
            validator
                .validate_and_parse("unknown")
                .unwrap()
                .cast::<String>(),
            "?"
        );
    }

    #[test]
    fn test_is_accepted_helper() {
        let validator = BoolValidator::new(false);

        assert!(validator.is_accepted("yes"));
        assert!(validator.is_accepted("no"));
        assert!(!validator.is_accepted("maybe"));
    }
}
