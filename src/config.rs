use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct RenameRule {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileRule {
    pub pattern: String,
    #[serde(default)]
    pub renames: Vec<RenameRule>,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub suffix: Option<String>,
    pub destination: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rules: Vec<FileRule>,
}

pub fn load_config() -> Result<Config, String> {
    let config_path = Path::new(".mover.json");
    
    if !config_path.exists() {
        return Err("Configuration file '.mover.json' not found in current directory.".to_string());
    }
    
    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read config file: {}", e)),
    };
    
    match serde_json::from_str::<Config>(&config_content) {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Failed to parse config file: {}", e)),
    }
}
