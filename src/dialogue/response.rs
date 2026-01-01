use std::collections::HashMap;

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
}

pub struct AnyValidator;

impl ResponseValidator for AnyValidator {
    type Output = String;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        Ok(self.clear_text(text))
    }
}

pub struct ListOrNoneValidator {
    allowed_values: Vec<String>,
    none_text: String,
    case_sensitive: bool,
}

impl ListOrNoneValidator {
    pub fn new(allowed_values: Vec<String>, none_text: String) -> Self {
        Self {
            allowed_values,
            none_text,
            case_sensitive: false,
        }
    }

    pub fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }
}

impl ResponseValidator for ListOrNoneValidator {
    type Output = Option<String>;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text);

        let compare_text = if self.case_sensitive {
            cleaned.clone()
        } else {
            cleaned.to_lowercase()
        };

        let compare_none = if self.case_sensitive {
            self.none_text.clone()
        } else {
            self.none_text.to_lowercase()
        };

        // Check for "none" first
        if compare_text.contains(&compare_none) {
            return Ok(None);
        }

        // Check against allowed values
        for allowed in &self.allowed_values {
            let compare_allowed = if self.case_sensitive {
                allowed.clone()
            } else {
                allowed.to_lowercase()
            };

            if compare_text.contains(&compare_allowed) {
                return Ok(Some(cleaned));
            }
        }

        Err(ValidationError::NotAccepted)
    }
}

pub struct OptionalValidator {
    none_text: String,
}

impl OptionalValidator {
    pub fn new(none_text: String) -> Self {
        Self { none_text }
    }
}

impl ResponseValidator for OptionalValidator {
    type Output = Option<String>;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text);
        let none_text_lower = self.none_text.to_lowercase();

        if cleaned.to_lowercase().contains(&none_text_lower) {
            Ok(None)
        } else {
            Ok(Some(cleaned))
        }
    }
}

pub struct BoolValidator {
    yes_text: String,
    no_text: String,
    always_text: String,
    never_text: String,
    hard_search: bool,
}

impl BoolValidator {
    pub fn new(yes_text: String, no_text: String, always_text: String, never_text: String) -> Self {
        Self {
            yes_text,
            no_text,
            always_text,
            never_text,
            hard_search: false,
        }
    }

    pub fn hard_search(mut self, enabled: bool) -> Self {
        self.hard_search = enabled;
        self
    }

    fn get_mappings(&self) -> HashMap<String, bool> {
        let mut mappings = HashMap::new();
        mappings.insert(self.yes_text.to_lowercase(), true);
        mappings.insert(self.no_text.to_lowercase(), false);
        mappings.insert(self.always_text.to_lowercase(), true);
        mappings.insert(self.never_text.to_lowercase(), false);
        mappings
    }
}

impl ResponseValidator for BoolValidator {
    type Output = bool;

    fn validate_and_parse(&self, text: &str) -> Result<Self::Output, ValidationError> {
        let cleaned = self.clear_text(text).to_lowercase();
        let mappings = self.get_mappings();

        if self.hard_search {
            // Exact match required
            mappings
                .get(&cleaned)
                .copied()
                .ok_or(ValidationError::NotAccepted)
        } else {
            // Partial match allowed
            for (key, &value) in &mappings {
                if cleaned.contains(key) {
                    return Ok(value);
                }
            }
            Err(ValidationError::NotAccepted)
        }
    }
}

pub struct MappedValidator<T: Clone> {
    mappings: HashMap<String, T>,
    default: Option<T>,
    hard_search: bool,
}

impl<T: Clone> MappedValidator<T> {
    pub fn new(mappings: HashMap<String, T>) -> Self {
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
            self.mappings.get(&cleaned)
        } else {
            self.mappings
                .iter()
                .find(|(key, _)| cleaned.contains(key.as_str()))
                .map(|(_, v)| v)
        };

        match result {
            Some(value) => Ok(value.clone()),
            None => self.default.clone().ok_or(ValidationError::NotAccepted),
        }
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
            "none".to_string(),
        );

        assert_eq!(validator.validate_and_parse("none").unwrap(), None);
        assert_eq!(validator.validate_and_parse("NONE").unwrap(), None);
        assert_eq!(validator.validate_and_parse("I want none").unwrap(), None);
    }

    #[test]
    fn test_list_or_none_validator_with_values() {
        let validator = ListOrNoneValidator::new(
            vec!["apple".to_string(), "banana".to_string()],
            "none".to_string(),
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
            "None".to_string(),
        ).case_sensitive(true);

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
            "none".to_string(),
        );

        assert!(validator.validate_and_parse("orange").is_err());
        assert!(validator.validate_and_parse("random text").is_err());
    }

    #[test]
    fn test_optional_validator_with_none() {
        let validator = OptionalValidator::new("none".to_string());

        assert_eq!(validator.validate_and_parse("none").unwrap(), None);
        assert_eq!(validator.validate_and_parse("NONE").unwrap(), None);
        assert_eq!(validator.validate_and_parse("I choose none").unwrap(), None);
    }

    #[test]
    fn test_optional_validator_with_value() {
        let validator = OptionalValidator::new("none".to_string());

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
        let validator = BoolValidator::new(
            "yes".to_string(),
            "no".to_string(),
            "always".to_string(),
            "never".to_string(),
        );

        assert_eq!(validator.validate_and_parse("yes").unwrap(), true);
        assert_eq!(validator.validate_and_parse("no").unwrap(), false);
        assert_eq!(validator.validate_and_parse("always").unwrap(), true);
        assert_eq!(validator.validate_and_parse("never").unwrap(), false);
    }

    #[test]
    fn test_bool_validator_case_insensitive() {
        let validator = BoolValidator::new(
            "yes".to_string(),
            "no".to_string(),
            "always".to_string(),
            "never".to_string(),
        );

        assert_eq!(validator.validate_and_parse("YES").unwrap(), true);
        assert_eq!(validator.validate_and_parse("No").unwrap(), false);
        assert_eq!(validator.validate_and_parse("ALWAYS").unwrap(), true);
        assert_eq!(validator.validate_and_parse("NeVeR").unwrap(), false);
    }

    #[test]
    fn test_bool_validator_partial_match() {
        let validator = BoolValidator::new(
            "yes".to_string(),
            "no".to_string(),
            "always".to_string(),
            "never".to_string(),
        );

        assert_eq!(validator.validate_and_parse("oh yes please").unwrap(), true);
        assert_eq!(validator.validate_and_parse("no way").unwrap(), false);
        assert_eq!(validator.validate_and_parse("I will always do it").unwrap(), true);
    }

    #[test]
    fn test_bool_validator_hard_search() {
        let validator = BoolValidator::new(
            "yes".to_string(),
            "no".to_string(),
            "always".to_string(),
            "never".to_string(),
        ).hard_search(true);

        assert_eq!(validator.validate_and_parse("yes").unwrap(), true);
        assert!(validator.validate_and_parse("oh yes please").is_err());
        assert!(validator.validate_and_parse("maybe").is_err());
    }

    #[test]
    fn test_bool_validator_not_accepted() {
        let validator = BoolValidator::new(
            "yes".to_string(),
            "no".to_string(),
            "always".to_string(),
            "never".to_string(),
        );

        assert!(validator.validate_and_parse("maybe").is_err());
        assert!(validator.validate_and_parse("sometimes").is_err());
        assert!(validator.validate_and_parse("").is_err());
    }

    #[test]
    fn test_bool_validator_custom_translations() {
        let validator = BoolValidator::new(
            "si".to_string(),      // Spanish yes
            "no".to_string(),      // Spanish no
            "siempre".to_string(), // Spanish always
            "nunca".to_string(),   // Spanish never
        );

        assert_eq!(validator.validate_and_parse("si").unwrap(), true);
        assert_eq!(validator.validate_and_parse("no").unwrap(), false);
        assert_eq!(validator.validate_and_parse("siempre").unwrap(), true);
        assert_eq!(validator.validate_and_parse("nunca").unwrap(), false);
    }

    #[test]
    fn test_mapped_validator_basic() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), 1);
        mappings.insert("blue".to_string(), 2);
        mappings.insert("green".to_string(), 3);

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("blue").unwrap(), 2);
        assert_eq!(validator.validate_and_parse("green").unwrap(), 3);
    }

    #[test]
    fn test_mapped_validator_with_default() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), 1);
        mappings.insert("blue".to_string(), 2);

        let validator = MappedValidator::new(mappings).with_default(0);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("unknown").unwrap(), 0);
        assert_eq!(validator.validate_and_parse("xyz").unwrap(), 0);
    }

    #[test]
    fn test_mapped_validator_without_default() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), 1);

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert!(validator.validate_and_parse("unknown").is_err());
    }

    #[test]
    fn test_mapped_validator_partial_match() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), 1);
        mappings.insert("blue".to_string(), 2);

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("I like red").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("dark blue color").unwrap(), 2);
    }

    #[test]
    fn test_mapped_validator_hard_search() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), 1);
        mappings.insert("blue".to_string(), 2);

        let validator = MappedValidator::new(mappings).hard_search(true);

        assert_eq!(validator.validate_and_parse("red").unwrap(), 1);
        assert!(validator.validate_and_parse("I like red").is_err());
        assert!(validator.validate_and_parse("dark blue").is_err());
    }

    #[test]
    fn test_mapped_validator_case_insensitive() {
        let mut mappings = HashMap::new();
        mappings.insert("red".to_string(), 1);

        let validator = MappedValidator::new(mappings);

        assert_eq!(validator.validate_and_parse("RED").unwrap(), 1);
        assert_eq!(validator.validate_and_parse("Red").unwrap(), 1);
    }

    #[test]
    fn test_mapped_validator_with_strings() {
        let mut mappings = HashMap::new();
        mappings.insert("small".to_string(), "S".to_string());
        mappings.insert("medium".to_string(), "M".to_string());
        mappings.insert("large".to_string(), "L".to_string());

        let validator = MappedValidator::new(mappings).with_default("?".to_string());

        assert_eq!(validator.validate_and_parse("small").unwrap(), "S");
        assert_eq!(validator.validate_and_parse("medium").unwrap(), "M");
        assert_eq!(validator.validate_and_parse("unknown").unwrap(), "?");
    }

    #[test]
    fn test_is_accepted_helper() {
        let validator = BoolValidator::new(
            "yes".to_string(),
            "no".to_string(),
            "always".to_string(),
            "never".to_string(),
        );

        assert!(validator.is_accepted("yes"));
        assert!(validator.is_accepted("no"));
        assert!(!validator.is_accepted("maybe"));
    }
}