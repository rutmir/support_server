use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use crate::{config::Config, error::AppError};
use std::sync::Arc;

/// Middleware function to validate API key in Authorization header
pub async fn api_key_auth(
    State(config): State<Arc<Config>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Extract the Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| AppError::new_bad_request())?
        .to_str()
        .map_err(|_| AppError::new_bad_request())?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::new_bad_request());
    }

    // Extract the token
    let token = &auth_header[7..];

    // Validate against configured API keys
    if !config.valid_api_keys.contains(&token.to_string()) {
        return Err(AppError::new_bad_request());
    }

    // If valid, continue with the request
    let response = next.run(request).await;
    Ok(response)
}