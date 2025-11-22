use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents a request to report an error from a client application
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ErrorReportRequest {
    /// Information about the application
    #[validate(nested)]
    pub app: App,
    /// Information about the device reporting the error
    #[validate(nested)]
    pub device: Device,
    /// Information about the error
    #[validate(nested)]
    pub error: Error,
}

/// Information about the error
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct App {
    /// Application name
    #[validate(length(min = 1))]
    pub name: String,
    /// application version
    #[validate(length(min = 1))]
    pub version: String,
}

/// Information about the device reporting the error
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Device {
    /// Unique identifier for the device
    #[validate(length(min = 1))]
    pub id: String,
    /// Description of the device (optional)
    pub description: Option<String>,
}

/// Information about the error
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Error {
    /// The error message
    #[validate(length(min = 1))]
    pub message: String,
    /// Stack trace information
    pub trace: Vec<String>,
}