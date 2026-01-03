use crate::ctx::runtime;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    ParseError(String),
    NotAccepted,
}

pub trait ResponseValidator {
    type Output;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError>;

    fn is_accepted(&self, text: &str) -> bool {
        self.validate_and_parse(text).is_ok()
    }

    fn clear_text(&self, text: &str) -> String {
        text.trim().to_string()
    }

    fn get_error_txt(&self, error: &ValidationError) -> String;
}

pub struct AnyValidator;

impl ResponseValidator for AnyValidator {
    type Output = String;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        Ok(self.clear_text(text))
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        "How?".to_string()
    }
}

pub struct ListOrNoneValidator {
    pub(crate) allowed_values: Vec<String>,
}

impl ListOrNoneValidator {
    pub fn new(allowed_values: Vec<String>) -> Self {
        Self {
            allowed_values
        }
    }
}

impl ResponseValidator for ListOrNoneValidator {
    type Output = Option<String>;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();

        let none_translations = runtime().language_system.get_translation_list("none");

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
        todo!()
    }
}

pub struct OptionalValidator;

impl ResponseValidator for OptionalValidator {
    type Output = Option<String>;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text);
        let cleaned_lower = cleaned.to_lowercase();

        let none_translations = runtime().language_system.get_translation_list("none");

        for none_text in none_translations {
            if cleaned_lower.contains(&none_text.to_lowercase()) {
                return Ok(None);
            }
        }
    
        Ok(Some(cleaned))
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        runtime().language_system.get_translation("error_validator_optional")
    }
}

pub struct BoolValidator {
    pub(crate) hard_search: bool,
}

impl BoolValidator {
    pub fn new(hard_search: bool) -> Self {
        Self {
            hard_search
        }
    }
}

impl ResponseValidator for BoolValidator {
    type Output = bool;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();

        let yes_translations = runtime().language_system.get_translation_list("yes");
        let no_translations = runtime().language_system.get_translation_list("no");

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
        todo!()
    }
}

pub struct MappedValidator<T: Clone> {
    pub(crate) mappings: Vec<(String, T)>,
    pub(crate) default: Option<T>,
    pub(crate) hard_search: bool,
}

impl<T: Clone> MappedValidator<T> {
    pub fn new(mappings: Vec<(String, T)>) -> Self {
        Self {
            mappings,
            default: None,
            hard_search: false,
        }
    }

    pub fn with_default(mut self, default: T) -> Self {
        self.default = Some(default);
        self
    }

    pub fn hard_search(mut self, enabled: bool) -> Self {
        self.hard_search = enabled;
        self
    }
}

impl<T: Clone> ResponseValidator for MappedValidator<T> {
    type Output = T;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();

        let result = if self.hard_search {
            self.mappings
                .iter()
                .find(|(key, _)| cleaned == key.to_lowercase())
                .map(|(_, v)| v)
        } else {
            self.mappings
                .iter()
                .find(|(key, _)| cleaned.contains(&key.to_lowercase()))
                .map(|(_, v)| v)
        };

        match result {
            Some(value) => Ok(value.clone()),
            None => self.default.clone().ok_or(ValidationError::NotAccepted),
        }
    }

    fn get_error_txt(&self, _error: &ValidationError) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_validator_basic() {
        let validator = AnyValidator;
        assert_eq!(validator.validate_and_parse("hello").unwrap(), "hello");
        assert_eq!(validator.validate_and_parse("  spaces  ").unwrap(), "spaces");
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
        let validator = ListOrNoneValidator::new(
            vec!["apple".to_string(), "banana".to_string()],
        );

        assert_eq!(validator.validate_and_parse("none").unwrap(), None);
        assert_eq!(validator.validate_and_parse("NONE").unwrap(), None);
        assert_eq!(validator.validate_and_parse("I want none").unwrap(), None);
    }

    #[test]
    fn test_list_or_none_validator_with_values() {
        let validator = ListOrNoneValidator::new(
            vec!["apple".to_string(), "banana".to_string()],
        );

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
        let validator = ListOrNoneValidator::new(
            vec!["Apple".to_string()],
        );

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
        let validator = ListOrNoneValidator::new(
            vec!["apple".to_string()],
        );

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
        assert_eq!(validator.validate_and_parse("I will always do it").unwrap(), true);
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
        let mappings = vec![
            ("red".to_string(), 1),
            ("blue".to_string(), 2),
            ("green".to_string(), 3),
        ];

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("blue").unwrap(), 2);
        assert_eq!(validator.validate_and_parse("green").unwrap(), 3);
    }

    #[test]
    fn test_mapped_validator_with_default() {
        let mappings = vec![
            ("red".to_string(), 1),
            ("blue".to_string(), 2),
        ];

        let validator = MappedValidator::new(mappings).with_default(0);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("unknown").unwrap(), 0);
        assert_eq!(validator.validate_and_parse("xyz").unwrap(), 0);
    }

    #[test]
    fn test_mapped_validator_without_default() {
        let mappings = vec![
            ("red".to_string(), 1),
        ];

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert!(validator.validate_and_parse("unknown").is_err());
    }

    #[test]
    fn test_mapped_validator_partial_match() {
        let mappings = vec![
            ("red".to_string(), 1),
            ("blue".to_string(), 2),
        ];

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("I like red").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("dark blue color").unwrap(), 2);
    }

    #[test]
    fn test_mapped_validator_hard_search() {
        let mappings = vec![
            ("red".to_string(), 1),
            ("blue".to_string(), 2),
        ];

        let validator = MappedValidator::new(mappings).hard_search(true);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert!(validator.validate_and_parse("I like red").is_err());
        assert!(validator.validate_and_parse("dark blue").is_err());
    }

    #[test]
    fn test_mapped_validator_case_insensitive() {
        let mappings = vec![
            ("red".to_string(), 1),
        ];

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("RED").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("Red").unwrap(), 1);
    }

    #[test]
    fn test_mapped_validator_with_strings() {
        let mappings = vec![
            ("small".to_string(), "S".to_string()),
            ("medium".to_string(), "M".to_string()),
            ("large".to_string(), "L".to_string()),
        ];

        let validator = MappedValidator::new(mappings).with_default("?".to_string());

        assert_eq!(validator.validate_and_parse("small").unwrap(), "S");
        assert_eq!(validator.validate_and_parse("medium").unwrap(), "M");
        assert_eq!(validator.validate_and_parse("unknown").unwrap(), "?");
    }

    #[test]
    fn test_is_accepted_helper() {
        let validator = BoolValidator::new(false);

        assert!(validator.is_accepted("yes"));
        assert!(validator.is_accepted("no"));
        assert!(!validator.is_accepted("maybe"));
    }
}