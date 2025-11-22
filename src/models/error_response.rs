use serde::{Deserialize, Serialize};

/// Represents a successful response from the API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuccessResponse {
    /// Status indicator
    pub status: String,
    /// Descriptive message
    pub message: String,
}

impl SuccessResponse {
    /// Create a new success response
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            status: "success".to_string(),
            message: message.into(),
        }
    }
}

/// Represents an error response from the API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    /// Status indicator
    pub status: String,
    /// Descriptive error message
    pub message: String,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            message: message.into(),
        }
    }
}