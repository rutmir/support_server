use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    error::AppError,
    models::ErrorReportRequest,
    state::AppState,
};
use validator::Validate;

/// Handle error report requests
pub async fn report_error(
    State(state): State<AppState>,
    Json(payload): Json<ErrorReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate the request payload
    payload.validate().map_err(|_| AppError::new_bad_request())?;
    
    // Process the error report
    state.error_report_service.handle_report(payload).await?;
    
    // Return success response
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Error report received and processed"
        }))
    ))
}