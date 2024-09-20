use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Once, ONCE_INIT};
extern crate config;
use config::{Config, File, ConfigError, Value};


static SETTINGS_FILES: &[&str] = &[
    "test_generation_prompt_htzr_cn.toml",
    "language_extensions.toml",
    "analyze_suite_test_headers_indentation.toml",
    "analyze_suite_test_insert_line.toml",
];

pub struct SingletonSettings {
    config: Config,
}

impl SingletonSettings {
    fn new() -> Result<Self, ConfigError> {
        let base_dir = env::current_exe()
            .map(|path| path.parent().unwrap().to_path_buf())
            .unwrap_or_else(|_| env::current_dir().unwrap());

        let settings_files: Vec<PathBuf> = SETTINGS_FILES
            .iter()
            .map(|&f| base_dir.join(f))
            .collect();

        // Ensure all settings files exist
        for file_path in &settings_files {
            if !file_path.exists() {
                return Err(ConfigError::Message(format!("Settings file not found: {}", file_path.to_string_lossy())));
            }
        }

        let mut config = Config::default();
        for file_path in settings_files {
            config.merge(File::from(file_path))?;
        }

        Ok(SingletonSettings { config })
    }

    pub fn get_language_extensions(&self) -> Result<Value, ConfigError> {
        self.config.get("language_extensions")
    }


    pub fn get_test_generation_prompt_htzr_cn(&self) -> Result<Value, ConfigError> {
        self.config.get("test_generation_prompt_htzr_cn")
    }

    

    pub fn get_analyze_suite_test_headers_indentation(&self) -> Result<Value, ConfigError> {
        self.config.get("analyze_suite_test_headers_indentation")
    }

    pub fn get_analyze_suite_test_insert_line(&self) -> Result<Value, ConfigError> {
        self.config.get("analyze_suite_test_insert_line")
    }

}

// Singleton implementation
static mut SINGLETON: Option<SingletonSettings> = None;
static INIT: Once = ONCE_INIT;

pub fn get_settings() -> &'static SingletonSettings {
    unsafe {
        INIT.call_once(|| {
            SINGLETON = Some(SingletonSettings::new().expect("Failed to initialize settings"));
        });
        SINGLETON.as_ref().unwrap()
    }
}

fn main() {
    let settings = get_settings();
    println!("{:?}", settings.get_test_generation_prompt_htzr_cn() );
}