use content_resolver::{error::Result, resolver::ResourceResolver, types::EntryType};
use std::boxed::Box;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs;

/// High-level interface for working with skills
///
/// Skills are collections of files and folders that can span
/// multiple directories and be downloaded recursively.
pub struct SkillProvider {
    resolver: Arc<ResourceResolver>,
    base_path: String,
}

/// Information about an available skill
#[derive(Debug, Clone)]
pub struct SkillInfo {
    /// Unique identifier for the skill
    pub id: String,
    /// Path to the skill directory
    #[allow(dead_code)]
    pub path: String,
}

/// Result of downloading a skill
#[derive(Debug)]
pub struct SkillDownload {
    /// The skill that was downloaded
    #[allow(dead_code)]
    pub skill: SkillInfo,
    /// Paths of all files that were written
    pub files_written: Vec<PathBuf>,
    /// Total bytes downloaded
    pub total_bytes: usize,
}
#[allow(dead_code)]
impl SkillProvider {
    /// Create a new skill provider
    ///
    /// # Arguments
    /// * `resolver` - The underlying resource resolver
    /// * `base_path` - Base directory where skills are stored
    pub fn new(resolver: Arc<ResourceResolver>, base_path: String) -> Self {
        Self {
            resolver,
            base_path,
        }
    }

    /// List all available skills
    ///
    /// Returns a list of skill IDs found in the base path
    pub async fn list_skills(&self) -> Result<Vec<SkillInfo>> {
        let listing = self.resolver.list_directory_merged(&self.base_path).await?;

        let mut skills = Vec::new();
        for entry in listing.entries {
            if entry.entry_type == EntryType::Dir {
                skills.push(SkillInfo {
                    id: entry.name.clone(),
                    path: entry.path,
                });
            }
        }

        skills.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(skills)
    }

    /// Check if a skill exists
    pub async fn skill_exists(&self, skill_id: &str) -> bool {
        let path = self.skill_path(skill_id);
        self.resolver.list_directory(&path).await.is_ok()
    }

    /// Download a skill to a local directory
    ///
    /// Recursively fetches all files and subdirectories, preserving structure
    ///
    /// # Arguments
    /// * `skill_id` - The skill identifier
    /// * `output_dir` - Local directory to write files to
    pub async fn download_skill(&self, skill_id: &str, output_dir: &Path) -> Result<SkillDownload> {
        let skill_path = self.skill_path(skill_id);

        // Verify the skill exists
        self.resolver.list_directory(&skill_path).await?;

        let mut files_written = Vec::new();
        let mut total_bytes = 0;

        // Download recursively
        self.download_directory_recursive(
            &skill_path,
            output_dir,
            &mut files_written,
            &mut total_bytes,
        )
        .await?;

        Ok(SkillDownload {
            skill: SkillInfo {
                id: skill_id.to_string(),
                path: skill_path,
            },
            files_written,
            total_bytes,
        })
    }

    /// Download a specific file from a skill
    pub async fn download_skill_file(
        &self,
        skill_id: &str,
        file_path: &str,
        output_path: &Path,
    ) -> Result<usize> {
        let full_path = format!(
            "{}/{}",
            self.skill_path(skill_id),
            file_path.trim_start_matches('/')
        );

        let content = self.resolver.fetch_file(&full_path).await?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(output_path, &content.content).await?;

        Ok(content.content.len())
    }

    /// Get the structure of a skill without downloading it
    ///
    /// Returns all file and directory paths within the skill
    pub async fn get_skill_structure(&self, skill_id: &str) -> Result<Vec<String>> {
        let skill_path = self.skill_path(skill_id);
        let mut structure = Vec::new();

        self.collect_structure_recursive(&skill_path, &mut structure)
            .await?;

        structure.sort();
        Ok(structure)
    }

    fn skill_path(&self, skill_id: &str) -> String {
        format!("{}/{}", self.base_path.trim_end_matches('/'), skill_id)
    }

    fn download_directory_recursive<'a>(
        &'a self,
        remote_path: &'a str,
        local_path: &'a Path,
        files_written: &'a mut Vec<PathBuf>,
        total_bytes: &'a mut usize,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            // Create the directory
            fs::create_dir_all(local_path).await?;
            // List contents

            let listing = self.resolver.list_directory(remote_path).await?;

            for entry in listing.entries {
                let local_entry_path = local_path.join(&entry.name);
                match entry.entry_type {
                    EntryType::File => {
                        // Download the file
                        let content = self.resolver.fetch_file(&entry.path).await?;
                        fs::write(&local_entry_path, &content.content).await?;
                        *total_bytes += content.content.len();
                        files_written.push(local_entry_path);
                    }
                    EntryType::Dir => {
                        // Recurse into subdirectory
                        self.download_directory_recursive(
                            &entry.path,
                            &local_entry_path,
                            files_written,
                            total_bytes,
                        )
                        .await?;
                    }
                }
            }
            Ok(())
        })
    }

    fn collect_structure_recursive<'a>(
        &'a self,
        remote_path: &'a str,
        structure: &'a mut Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let listing = self.resolver.list_directory(remote_path).await?;
            for entry in listing.entries {
                structure.push(entry.path.clone());
                if entry.entry_type == EntryType::Dir {
                    self.collect_structure_recursive(&entry.path, structure)
                        .await?;
                }
            }
            Ok(())
        })
    }
}
