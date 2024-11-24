use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Runner;

impl Runner {
    /// Executes a shell command and returns its output along with timing information.
    ///
    /// # Arguments
    /// * `command` - The shell command to execute
    /// * `cwd` - Optional working directory in which to execute the command
    ///
    /// # Returns
    /// Returns a tuple containing:
    /// * stdout: String - The standard output
    /// * stderr: String - The standard error
    /// * exit_code: i32 - The exit code
    /// * command_start_time: u128 - The timestamp when the command started (in milliseconds)
    pub fn run_command(command: &str, cwd: Option<&str>) -> (String, String, i32, u128) {
        // Get current time in milliseconds
        let command_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // Create command
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(command);

        // Set working directory if provided
        if let Some(working_dir) = cwd {
            cmd.current_dir(working_dir);
        }

        // Execute command
        let output = cmd.output().expect("Failed to execute command");

        // Convert output to strings
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let exit_code = output.status.code().unwrap_or(-1);

        (stdout, stderr, exit_code, command_start_time)
    }
}
