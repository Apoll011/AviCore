use std::sync::Arc;

use content_resolver::{
    error::{ContentError, Result},
    resolver::ResourceResolver,
};

/// High-level interface for fetching language files
///
/// Language files are identified by locale codes (e.g., "en", "pt-PT")
/// and have a .lang extension.
pub struct LanguageProvider {
    resolver: Arc<ResourceResolver>,
    base_path: String,
}

#[allow(dead_code)]
impl LanguageProvider {
    /// Create a new language provider
    ///
    /// # Arguments
    /// * `resolver` - The underlying resource resolver
    /// * `base_path` - Base directory where language files are stored
    pub fn new(resolver: Arc<ResourceResolver>, base_path: String) -> Self {
        Self {
            resolver,
            base_path,
        }
    }

    /// Fetch a language file by locale code
    ///
    /// # Arguments
    /// * `locale` - The locale code (e.g., "en", "pt-PT", "zh-CN")
    ///
    /// # Returns
    /// The raw file contents as a String, or an error if not found
    pub async fn fetch_language(&self, locale: &str) -> Result<String> {
        let path = self.language_path(locale);
        let content = self.resolver.fetch_file(&path).await?;

        String::from_utf8(content.content.to_vec()).map_err(|e| ContentError::InvalidStructure {
            message: format!("Language file is not valid UTF-8: {}", e),
        })
    }

    /// Check if a language exists
    pub async fn language_exists(&self, locale: &str) -> bool {
        let path = self.language_path(locale);
        self.resolver.file_exists(&path).await
    }

    /// List all available languages
    ///
    /// Returns locale codes for all .lang files found
    pub async fn list_languages(&self) -> Result<Vec<String>> {
        let listing = self.resolver.list_directory_merged(&self.base_path).await?;

        let mut locales = Vec::new();
        for entry in listing.entries {
            if entry.name.ends_with(".lang")
                && let Some(locale) = entry.name.strip_suffix(".lang")
            {
                locales.push(locale.to_string());
            }
        }

        locales.sort();
        Ok(locales)
    }

    /// Fetch a language with fallback to a default locale
    ///
    /// If the requested locale is not found, falls back to the default.
    /// Useful for locale chains like "pt-BR" → "pt" → "en"
    pub async fn fetch_with_fallback(&self, locale: &str, fallback: &str) -> Result<String> {
        match self.fetch_language(locale).await {
            Ok(content) => Ok(content),
            Err(ContentError::NotFound { .. }) => self.fetch_language(fallback).await,
            Err(e) => Err(e),
        }
    }

    /// Fetch a language with multiple fallbacks
    ///
    /// Tries each locale in order until one succeeds
    pub async fn fetch_with_fallbacks(&self, locales: &[&str]) -> Result<String> {
        for locale in locales {
            match self.fetch_language(locale).await {
                Ok(content) => return Ok(content),
                Err(ContentError::NotFound { .. }) => continue,
                Err(e) => return Err(e),
            }
        }

        Err(ContentError::NotFound {
            path: format!("No language found for locales: {:?}", locales),
        })
    }

    fn language_path(&self, locale: &str) -> String {
        format!("{}/{}.lang", self.base_path.trim_end_matches('/'), locale)
    }
}
