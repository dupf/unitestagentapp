use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Runner;

impl Runner {
    pub fn run_command(command: &str, cwd: Option<&str>) -> (String, String, i32, u128) {
        // Get the current time before running the test command, in milliseconds
        let command_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        // Create a new Command
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(command);

        // Set the working directory if provided
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        // Execute the command
        let output = cmd.output().expect("Failed to execute command");

        // Convert stdout and stderr to strings
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        // Return the tuple with the desired information
        (stdout, stderr, output.status.code().unwrap_or(-1), command_start_time)
    }
}