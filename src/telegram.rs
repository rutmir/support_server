use crate::{config::Config, error::AppError};
use reqwest::Client;
use serde::Serialize;

/// Telegram API client for sending notifications
pub struct TelegramClient {
    client: Client,
    bot_token: String,
    chat_id: String,
}

impl TelegramClient {
    /// Create a new Telegram client with the provided configuration
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            bot_token: config.telegram_bot_token.clone(),
            chat_id: config.telegram_chat_id.clone(),
        }
    }

    /// Send a notification message to the configured Telegram chat
    pub async fn send_notification(&self, message: &str) -> Result<(), AppError> {
        // Skip if bot token or chat ID is not configured
        if self.bot_token.is_empty() || self.chat_id.is_empty() {
            log::warn!("Telegram bot token or chat ID not configured, skipping notification");
            return Ok(());
        }

        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let request = SendMessageRequest {
            chat_id: &self.chat_id,
            text: message,
            parse_mode: "HTML",
        };

        self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(AppError::from)?
            .error_for_status()
            .map_err(|e| {
                AppError::new_external_service(format!(
                    "Failed to send Telegram message: {}",
                    e
                ))
            })?;

        log::info!("Successfully sent notification to Telegram chat {}", self.chat_id);
        Ok(())
    }
}

/// Request structure for sending a message via Telegram Bot API
#[derive(Serialize)]
struct SendMessageRequest<'a> {
    chat_id: &'a str,
    text: &'a str,
    parse_mode: &'a str,
}