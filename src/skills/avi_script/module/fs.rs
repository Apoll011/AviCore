use rhai::plugin::*;
use rhai::{EvalAltResult, Position};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[export_module]
pub mod fs_module {
    /// Reads the entire contents of a file as a ImmutableString
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Returns
    /// The file contents as a ImmutableString, or UNIT if the file could not be read
    #[rhai_fn(return_raw)]
    pub fn read(path: ImmutableString) -> Result<ImmutableString, Box<EvalAltResult>> {
        match fs::read_to_string(path.as_str()) {
            Ok(content) => Ok(ImmutableString::from(content)),
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Could not read directory: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Writes a ImmutableString to a file, overwriting its contents
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    /// * `content` - The ImmutableString to write to the file
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(return_raw)]
    pub fn write(
        path: ImmutableString,
        content: ImmutableString,
    ) -> Result<(), Box<EvalAltResult>> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.as_str())
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    format!("Could not open file: {}", e).into(),
                    Position::NONE,
                ))
            })?;

        write!(file, "{}", content).map_err(|e| {
            Box::new(EvalAltResult::ErrorRuntime(
                format!("Could not write to file: {}", e).into(),
                Position::NONE,
            ))
        })?;

        Ok(())
    }

    /// Appends a ImmutableString to the end of a file
    ///
    /// # Arguments
    /// * `path` - The path to the file to append to
    /// * `content` - The ImmutableString to append to the file
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(return_raw)]
    pub fn append(
        path: ImmutableString,
        content: ImmutableString,
    ) -> Result<(), Box<EvalAltResult>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.as_str())
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    format!("Could not open file: {}", e).into(),
                    Position::NONE,
                ))
            })?;

        write!(file, "{}", content).map_err(|e| {
            Box::new(EvalAltResult::ErrorRuntime(
                format!("Could not write to file: {}", e).into(),
                Position::NONE,
            ))
        })?;

        Ok(())
    }

    /// Checks if a path exists
    ///
    /// # Arguments
    /// * `path` - The path to check
    ///
    /// # Returns
    /// True if the path exists, false otherwise

    pub fn exists(path: ImmutableString) -> bool {
        Path::new(path.as_str()).exists()
    }

    /// Deletes a file or an empty directory
    ///
    /// # Arguments
    /// * `path` - The path to delete
    ///
    /// # Returns
    /// True if the deletion was successful, false otherwise

    pub fn delete(path: ImmutableString) -> bool {
        let path = Path::new(path.as_str());
        if path.is_dir() {
            fs::remove_dir(path).is_ok()
        } else {
            fs::remove_file(path).is_ok()
        }
    }

    /// Copies a file from one path to another
    ///
    /// # Arguments
    /// * `src` - The source path
    /// * `dest` - The destination path
    ///
    /// # Returns
    /// True if the copy was successful, false otherwise

    pub fn copy(src: ImmutableString, dest: ImmutableString) -> bool {
        fs::copy(src.as_str(), dest.as_str()).is_ok()
    }

    /// Moves or renames a file or directory
    ///
    /// # Arguments
    /// * `src` - The source path
    /// * `dest` - The destination path
    ///
    /// # Returns
    /// True if the move was successful, false otherwise
    #[rhai_fn(name = "move")]
    pub fn move_file(src: ImmutableString, dest: ImmutableString) -> bool {
        fs::rename(src.as_str(), dest.as_str()).is_ok()
    }

    /// Lists the names of files and directories in a given path
    ///
    /// # Arguments
    /// * `path` - The path to list
    ///
    /// # Returns
    /// A list of file and directory names
    #[rhai_fn(return_raw)]
    pub fn list_files(path: ImmutableString) -> Result<Vec<ImmutableString>, Box<EvalAltResult>> {
        let mut files = Vec::new();

        match fs::read_dir(path.as_str()) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(name) = entry.file_name().into_string() {
                            files.push(ImmutableString::from(name));
                        }
                    }
                }
                Ok(files)
            }
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Could not read directory: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Creates a directory and all its parent directories if they don't exist
    ///
    /// # Arguments
    /// * `path` - The path of the directory to create
    ///
    /// # Returns
    /// True if the directory was created successfully, false otherwise

    pub fn mkdir(path: ImmutableString) -> bool {
        fs::create_dir_all(path.as_str()).is_ok()
    }

    /// Gets the last component of a path
    ///
    /// # Arguments
    /// * `path` - The path to process
    ///
    /// # Returns
    /// The last component of the path

    pub fn basename(path: ImmutableString) -> ImmutableString {
        ImmutableString::from(
            Path::new(path.as_str())
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(""),
        )
    }

    /// Gets the parent directory of a path
    ///
    /// # Arguments
    /// * `path` - The path to process
    ///
    /// # Returns
    /// The parent directory of the path

    pub fn dirname(path: ImmutableString) -> ImmutableString {
        ImmutableString::from(
            Path::new(path.as_str())
                .parent()
                .and_then(|s| s.to_str())
                .unwrap_or(""),
        )
    }
}
