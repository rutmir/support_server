use config::{Config as ConfigLoader, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_server_address")]
    pub server_address: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    #[serde(default = "default_cache_ttl_minutes")]
    pub cache_ttl_minutes: u64,
    #[serde(default)]
    pub valid_api_keys: Vec<String>,
}

fn default_server_address() -> String {
    "127.0.0.1:8080".to_string()
}

fn default_cache_ttl_minutes() -> u64 {
    10
}

impl Config {
    pub fn from_env_and_file() -> Result<Self, config::ConfigError> {
        let config_dir = env::var("CONFIG_DIR").unwrap_or_else(|_| "./config".to_string());
        
        let config_builder = ConfigLoader::builder()
            // Start with default values
            .add_source(File::with_name(&format!("{}/default", config_dir)).required(false))
            // Add in environment variables prefixed with APP_
            .add_source(Environment::with_prefix("APP").separator("_"));

        let config = config_builder.build()?;
        config.try_deserialize()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: default_server_address(),
            telegram_bot_token: String::new(),
            telegram_chat_id: String::new(),
            cache_ttl_minutes: default_cache_ttl_minutes(),
            valid_api_keys: Vec::new(),
        }
    }
}