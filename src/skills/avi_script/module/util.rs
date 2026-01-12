use rhai::plugin::*;
use rhai::{EvalAltResult, Position};
use std::process::Command;
use uuid::Uuid;

#[export_module]
pub mod util_module {
    /// Generates a new UUID v4
    ///
    /// # Returns
    /// A UUID string in the format "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    pub fn uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// Asserts a condition, throwing an error if the condition is true
    ///
    /// # Arguments
    /// * `condition` - If true, an error will be thrown
    /// * `msg` - The error message to use
    ///
    /// # Note
    /// This has inverted logic compared to typical assertions - it errors when true
    #[rhai_fn(return_raw)]
    pub fn assert(condition: bool, msg: String) -> Result<(), Box<EvalAltResult>> {
        if condition {
            Err(Box::new(EvalAltResult::ErrorRuntime(
                msg.into(),
                Position::NONE,
            )))
        } else {
            Ok(())
        }
    }

    /// Executes a shell command and returns the exit code
    ///
    /// # Arguments
    /// * `command` - The command to execute
    ///
    /// # Returns
    /// The exit code of the command, or -1 if the code couldn't be determined
    ///
    /// # Note
    /// Uses "cmd /C" on Windows, "sh -c" on Unix-like systems
    #[rhai_fn(return_raw)]
    pub fn cmd(command: String) -> Result<i64, Box<EvalAltResult>> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", &command]).output()
        } else {
            Command::new("sh").arg("-c").arg(&command).output()
        };

        match output {
            Ok(output) => match output.status.code() {
                Some(code) => Ok(code as i64),
                None => Ok(-1),
            },
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to execute command: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the current operating system name
    ///
    /// # Returns
    /// The OS name (e.g., "linux", "windows", "macos")
    pub fn os() -> String {
        std::env::consts::OS.to_string()
    }

    /// Gets an environment variable with a default fallback
    ///
    /// # Arguments
    /// * `name` - The name of the environment variable
    /// * `default` - The default value to return if the variable is not set
    ///
    /// # Returns
    /// The environment variable value, or the default if not found
    pub fn env(name: String, default: String) -> String {
        std::env::var(name).unwrap_or(default)
    }

    pub fn get_string(data: Vec<u8>) -> String {
        String::from_utf8_lossy(&data).to_string()
    }
}
