use config::{Config as ConfigLoader, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_server_address")]
    pub server_address: String,
    #[serde(default)]
    pub telegram_bot_token: String,
    #[serde(default)]
    pub telegram_chat_id: String,
    /// Telegram Bot API base URL. Override for local e2e runs against a mock
    /// Telegram API (`APP_TELEGRAM_API_BASE`).
    #[serde(default = "default_telegram_api_base")]
    pub telegram_api_base: String,
    #[serde(default = "default_cache_ttl_minutes")]
    pub cache_ttl_minutes: u64,
    #[serde(default)]
    pub api_keys: Vec<String>,
}

fn default_server_address() -> String {
    "127.0.0.1:8080".to_string()
}

fn default_cache_ttl_minutes() -> u64 {
    10
}

fn default_telegram_api_base() -> String {
    "https://api.telegram.org".to_string()
}

impl Config {
    pub fn from_env_and_file() -> Result<Self, config::ConfigError> {
        let config_builder = ConfigLoader::builder()
            // Start with default values
            .add_source(File::with_name("./config").required(false))
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
            telegram_api_base: default_telegram_api_base(),
            cache_ttl_minutes: default_cache_ttl_minutes(),
            api_keys: Vec::new(),
        }
    }
}
