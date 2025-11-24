use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use config::ConfigError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalError,
    BadRequest,
    Unauthorized,
    ConfigError(String),
    ExternalServiceError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalError => write!(f, "Internal server error"),
            AppError::BadRequest => write!(f, "Bad request"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
        }
    }
}

impl AppError {
    pub fn new_internal() -> Self {
        AppError::InternalError
    }

    pub fn new_bad_request() -> Self {
        AppError::BadRequest
    }

    pub fn new_unauthorized() -> Self {
        AppError::Unauthorized
    }

    pub fn new_config<S: AsRef<str>>(msg: S) -> Self {
        AppError::ConfigError(msg.as_ref().to_string())
    }

    pub fn new_external_service<S: AsRef<str>>(msg: S) -> Self {
        AppError::ExternalServiceError(msg.as_ref().to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
        };

        let body = serde_json::json!({
            "error": self.to_string(),
        });

        (status, Json(body)).into_response()
    }
}

impl std::error::Error for AppError {}

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        AppError::new_config(error.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::new_external_service(error.to_string())
    }
}