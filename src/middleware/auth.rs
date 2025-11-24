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
        .ok_or_else(|| AppError::new_unauthorized())?
        .to_str()
        .map_err(|_| AppError::new_unauthorized())?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::new_unauthorized());
    }

    // Extract the token
    let token = &auth_header[7..];

    // Validate against configured API keys
    if !config.api_keys.contains(&token.to_string()) {
        return Err(AppError::new_unauthorized());
    }

    // If valid, continue with the request
    let response = next.run(request).await;
    Ok(response)
}