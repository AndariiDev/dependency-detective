use crate::DetectiveError; // Look in the "crate" (project) root for the error
use serde::{Deserialize, Serialize}; // Bring the actual macros into scope
use std::fs;
use std::path::Path; // Bring the specific type into scope

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsingRules {
    pub filenames: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub rules: Option<ParsingRules>,
}

// Write out instead of deriving to adjust default behavior:
impl Default for ParsingRules {
    fn default() -> Self {
        Self {
            filenames: vec!["main.c".to_string()],
        }
    }
}

impl Config {
    pub fn load(root: &Path) -> Result<Self, DetectiveError> {
        let config_path = root.join("detective.toml");

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?; // Read file
            let parsed_config: Config = toml::from_str(&content)?; // Parse string
            Ok(parsed_config)
        } else {
            Ok(Config::default())
        }
    }
}
