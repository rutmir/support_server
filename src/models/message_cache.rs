use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a cached error message to prevent duplicates
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageCacheEntry {
    /// Hash of the error message content
    pub message_hash: String,
    /// ID of the device that sent the message
    pub device_id: String,
    /// When the message was cached
    pub created_at: DateTime<Utc>,
    /// When the cache entry expires (10 minutes after creation)
    pub expires_at: DateTime<Utc>,
}