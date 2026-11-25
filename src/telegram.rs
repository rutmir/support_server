use crate::{config::Config, error::AppError};
use reqwest::Client;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

/// Boxed future used by the [`TelegramSender`] trait.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Abstraction over the Telegram transport so services can be unit-tested with
/// a mock and the running process can use the real Bot API.
pub trait TelegramSender: Send + Sync {
    fn send<'a>(&'a self, message: &'a str) -> BoxFuture<'a, Result<(), AppError>>;
}

/// Telegram API client for sending notifications.
pub struct TelegramClient {
    client: Client,
    bot_token: String,
    chat_id: String,
    /// Overridable for local e2e tests against a mock Telegram API
    /// (`APP_TELEGRAM_API_BASE=http://127.0.0.1:PORT`).
    api_base: String,
}

impl TelegramClient {
    /// Create a new Telegram client with the provided configuration.
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            bot_token: config.telegram_bot_token.clone(),
            chat_id: config.telegram_chat_id.clone(),
            api_base: config.telegram_api_base.clone(),
        }
    }

    /// Send a notification message to the configured Telegram chat.
    pub async fn send_notification(&self, message: &str) -> Result<(), AppError> {
        // Skip if bot token or chat ID is not configured
        if self.bot_token.is_empty() || self.chat_id.is_empty() {
            log::warn!("Telegram bot token or chat ID not configured, skipping notification");
            return Ok(());
        }

        let url = format!(
            "{}/bot{}/sendMessage",
            self.api_base.trim_end_matches('/'),
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

impl TelegramSender for TelegramClient {
    fn send<'a>(&'a self, message: &'a str) -> BoxFuture<'a, Result<(), AppError>> {
        Box::pin(self.send_notification(message))
    }
}

/// In-memory mock transport for tests and local e2e runs. Records every
/// message it is asked to deliver; can be configured to fail.
#[derive(Default, Clone)]
pub struct MockTelegramClient {
    sent: std::sync::Arc<Mutex<Vec<String>>>,
    fail: bool,
}

impl MockTelegramClient {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mock that fails every send (to test 502 propagation).
    pub fn failing() -> Self {
        Self {
            sent: std::sync::Arc::new(Mutex::new(Vec::new())),
            fail: true,
        }
    }

    /// Messages "delivered" so far.
    pub fn sent(&self) -> Vec<String> {
        self.sent.lock().expect("mock telegram mutex").clone()
    }

    pub fn count(&self) -> usize {
        self.sent.lock().expect("mock telegram mutex").len()
    }
}

impl TelegramSender for MockTelegramClient {
    fn send<'a>(&'a self, message: &'a str) -> BoxFuture<'a, Result<(), AppError>> {
        Box::pin(async move {
            if self.fail {
                return Err(AppError::new_external_service("mock telegram failure"));
            }
            self.sent
                .lock()
                .expect("mock telegram mutex")
                .push(message.to_string());
            Ok(())
        })
    }
}

impl Default for TelegramClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            bot_token: String::new(),
            chat_id: String::new(),
            api_base: "https://api.telegram.org".to_string(),
        }
    }
}

/// Request structure for sending a message via Telegram Bot API
#[derive(Serialize)]
struct SendMessageRequest<'a> {
    chat_id: &'a str,
    text: &'a str,
    parse_mode: &'a str,
}
