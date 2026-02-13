use log::{LevelFilter, SetLoggerError};
use simplelog::{
    CombinedLogger, ConfigBuilder, WriteLogger, TermLogger,
    TerminalMode, ColorChoice, Config,
};
use std::fs::File;

pub struct CustomLogger;

impl CustomLogger {
    pub fn init(name: &str) -> Result<(), SetLoggerError> {
        // Create config with timestamp, level, and target (name) formatting
        let config = ConfigBuilder::new()
            // .set_time_format_str("%Y-%m-%d %H:%M:%S")
            .set_target_level(LevelFilter::Info)
            .set_thread_level(LevelFilter::Off)
            .build();

        // Initialize combined logger with both file and terminal output
        CombinedLogger::init(vec![
            // File logger
            WriteLogger::new(
                LevelFilter::Info,
                config.clone(),
                File::create("run.log").unwrap(),
            ),
            // Terminal logger
            TermLogger::new(
                LevelFilter::Info,
                config,
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
        ])?;

        Ok(())
    }
}
