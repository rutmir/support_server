use crate::{
    cache::MessageCache, error::AppError, models::ErrorReportRequest,
    telegram::TelegramSender,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Service for handling error reports
pub struct ErrorReportService {
    telegram_client: Arc<dyn TelegramSender>,
    message_cache: MessageCache,
}

impl ErrorReportService {
    /// Create a new ErrorReportService
    pub fn new(telegram_client: Arc<dyn TelegramSender>, message_cache: MessageCache) -> Self {
        Self {
            telegram_client,
            message_cache,
        }
    }

    /// Generate a unique hash for an error message
    /// Combines device ID and error message to create a unique identifier
    pub fn generate_message_hash(&self, request: &ErrorReportRequest) -> String {
        let mut hasher = Sha256::new();
        hasher.update(request.error.message.as_bytes());
        hasher.update(request.app.name.as_bytes());
        hasher.update(request.app.version.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Handle an error report request
    ///
    /// This method formats the report data into a message suitable for Telegram
    /// and sends it via the Telegram client.
    pub async fn handle_report(&self, request: ErrorReportRequest) -> Result<(), AppError> {
        // Generate a hash for duplicate detection
        let message_hash = self.generate_message_hash(&request);

        // Check if this is a duplicate message
        if self.message_cache.get(&message_hash).await.is_some() {
            // Duplicate message - silently ignore as per contract
            log::info!("Duplicate error report detected and skipped for device: {}, hash: {}", request.device.id, message_hash);
            return Ok(());
        }

        // Format the report data into a message suitable for Telegram
        let message = self.format_report_message(&request);

        // Log before sending notification
        log::info!("Processing new or expired error report for device: {}", request.device.id);

        // Send the notification via Telegram
        self.telegram_client.send(&message).await?;

        // Cache the message hash to prevent duplicates
        self.message_cache.insert(message_hash, ()).await;

        Ok(())
    }

    /// Format an error report into a readable Telegram (HTML) message
    fn format_report_message(&self, request: &ErrorReportRequest) -> String {
        let mut message = String::new();

        message.push_str("<b>🚨 Error Report</b>\n\n");
        message.push_str(&format!(
            "<b>App:</b> {} v{}\n",
            escape_html(&request.app.name),
            escape_html(&request.app.version)
        ));
        message.push_str(&format!(
            "<b>Device:</b> {}\n",
            escape_html(&request.device.id)
        ));
        if let Some(description) = &request.device.description {
            message.push_str(&format!(
                "<b>Description:</b> {}\n",
                escape_html(description)
            ));
        }

        message.push_str(&format!(
            "\n<b>Error:</b> {}\n",
            escape_html(&request.error.message)
        ));

        if !request.error.trace.is_empty() {
            message.push_str("\n<b>Stack trace:</b>\n<pre>");
            for line in &request.error.trace {
                message.push_str(&escape_html(line));
                message.push('\n');
            }
            message.push_str("</pre>");
        }

        message
    }
}

/// Minimal HTML escape for Telegram `parse_mode=HTML`.
fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}
