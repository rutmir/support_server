use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request for the Argus notification gateway (`POST /api/v1/notify`).
///
/// This is the payload the Argus dispatcher notifier sends to the support
/// server (see `crates/dispatcher/src/notifier.rs`,
/// `SupportServerPayload`). All fields except `app`, `severity`, `title` and
/// `message` are optional so both the dispatcher and other consumers can use
/// the endpoint.
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct NotifyRequest {
    /// Producing application, e.g. `argus`.
    #[validate(length(min = 1, message = "app must not be empty"))]
    pub app: String,
    /// Severity: `info` | `warning` | `critical` (free-form accepted).
    #[validate(length(min = 1, message = "severity must not be empty"))]
    pub severity: String,
    /// Short human-readable title (Telegram bold line).
    #[validate(length(min = 1, message = "title must not be empty"))]
    pub title: String,
    /// Notification body.
    #[validate(length(min = 1, message = "message must not be empty"))]
    pub message: String,
    /// Related references (job ids, urls, paths). Sent in order.
    #[serde(default)]
    pub refs: Vec<String>,
    /// Optional primary reference/url (rendered in the message).
    #[serde(default)]
    pub url: Option<String>,
    /// Optional producer/source tag, e.g. `argus-dispatcher`.
    #[serde(default)]
    pub source: Option<String>,
    /// Optional Argus job id (kept for correlation and dedupe).
    #[serde(default)]
    pub job_id: Option<String>,
}

/// Response of `POST /api/v1/notify`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifyResponse {
    pub status: String,
    /// `true` when the exact same notification was already delivered inside
    /// the TTL window and was suppressed by dedupe.
    pub duplicate: bool,
    pub message: String,
    /// SHA-256 dedupe key (hex) — useful for debugging/tracing.
    pub dedupe_key: String,
}

impl NotifyResponse {
    pub fn accepted(dedupe_key: String) -> Self {
        Self {
            status: "success".to_string(),
            duplicate: false,
            message: "notification accepted".to_string(),
            dedupe_key,
        }
    }

    pub fn duplicate(dedupe_key: String) -> Self {
        Self {
            status: "success".to_string(),
            duplicate: true,
            message: "duplicate notification ignored (dedupe TTL window)".to_string(),
            dedupe_key,
        }
    }
}
