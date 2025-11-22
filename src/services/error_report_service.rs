use crate::{
    models::ErrorReportRequest,
    telegram::TelegramClient,
    error::AppError,
    cache::MessageCache,
};
use sha2::{Sha256, Digest};

/// Service for handling error reports
pub struct ErrorReportService {
    telegram_client: TelegramClient,
    message_cache: MessageCache,
}

impl ErrorReportService {
    /// Create a new ErrorReportService
    pub fn new(telegram_client: TelegramClient, message_cache: MessageCache) -> Self {
        Self {
            telegram_client,
            message_cache,
        }
    }

    /// Generate a unique hash for an error message
    /// Combines device ID and error message to create a unique identifier
    fn generate_message_hash(&self, request: &ErrorReportRequest) -> String {
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
        self.telegram_client.send_notification(&message).await?;
        
        // Cache the message hash to prevent duplicates
        self.message_cache.insert(message_hash, ()).await;
        
        // Log successful notification delivery
        log::info!("Error report notification successfully sent for device: {}", request.device.id);
        
        Ok(())
    }

    /// Format an error report into a human-readable message for Telegram
    fn format_report_message(&self, request: &ErrorReportRequest) -> String {
        let app_info = format!("{} ({})", request.app.name, request.app.version);

        let device_info = if let Some(description) = &request.device.description {
            format!("{} ({})", request.device.id, description)
        } else {
            request.device.id.clone()
        };

        let trace = if request.error.trace.is_empty() {
            "No stack trace provided".to_string()
        } else {
            request.error.trace.join("\n")
        };

        format!(
            "🚨 <b>New Error Report</b> 🚨\n\n\
             <b>Application:</b> {}\n\
             <b>Device:</b> {}\n\
             <b>Error Message:</b> {}\n\
             <b>Stack Trace:</b>\n<pre>{}</pre>",
            app_info,
            device_info,
            request.error.message,
            trace
        )
    }
}