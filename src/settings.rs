use std::collections::HashMap;
use directories::ProjectDirs;
use config::{Config, ConfigError, File};
use std::process;

pub struct Settings {
    /// The directory that contains all the journal files
    journal_dir: String
}

impl Settings {
    /// Returns a struct with the current settings
    ///
    /// If the settings file doesn't exist, it will be created.
    pub fn new() -> Settings {
        let proj_dirs = ProjectDirs::from("org", "jours", "jours");
        let mut settings = Config::default();

        let config_file_name = proj_dirs.config_dir().join("config.toml");
        let config_file_name = config_file_name.to_str()
            .expect("Path contains invalid UTF-8");

        match settings.merge(File::with_name(config_file_name)) {
            Ok(settings) => {
                let parsed_settings = settings.clone().try_into::<HashMap<String, String>>().unwrap();
                match parsed_settings.get("journal_dir") {
                    Some(journal_dir) => {
                        Settings { journal_dir: journal_dir.to_string() }
                    },
                    None => panic!("config.toml doesn't include 'journal_dir' key")
                }
            },
            Err(err) => {
                match err {
                    ConfigError::Frozen => {
                        eprintln!("Configuration was frozen and could not be modified");
                    }
                    ConfigError::NotFound(prop) => {
                        eprintln!("Configuration property '{}' not found", prop);
                    },
                    ConfigError::PathParse(error_kind) => {
                        eprintln!("Could not parse path for reason: {}", error_kind.description());
                    },
                    ConfigError::FileParse { uri, cause } => {
                        eprintln!("Configuration could not be parsed from file: '{}', cause: {}", uri.unwrap(), cause);
                    },
                    ConfigError::Type { origin, unexpected, expected, key } => {
                        eprintln!("Found '{}' but exectped '{}' for key '{}'", unexpected, expected, key.unwrap())
                    },
                    ConfigError::Message(msg) => {
                        eprintln!("Configuration property '{}' not found", msg);
                    },
                    ConfigError::Foreign(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
                process::exit(1);
            }
        }
    }
}
