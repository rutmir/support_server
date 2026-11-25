use crate::{error::AppError, models::{NotifyRequest, NotifyResponse}, state::AppState};
use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

/// `POST /api/v1/notify` — gateway used by the Argus dispatcher notifier
/// (and any other client) to deliver a notification to Telegram.
///
/// Auth: `Authorization: Bearer <api_key>` (same middleware as
/// `/api/v1/error-report`). Responses:
///
/// * `200 {"status":"success","duplicate":false,...}` — delivered to Telegram;
/// * `200 {"status":"success","duplicate":true,...}` — suppressed by dedupe
///   (identical notification inside the TTL window);
/// * `400` — payload failed validation (empty app/severity/title/message);
/// * `401` — missing/invalid bearer key;
/// * `502` — Telegram API rejected/failed the send.
pub async fn notify(
    State(state): State<AppState>,
    Json(payload): Json<NotifyRequest>,
) -> Result<(StatusCode, Json<NotifyResponse>), AppError> {
    if let Err(errors) = payload.validate() {
        log::warn!("invalid notify payload: {errors}");
        return Err(AppError::new_bad_request());
    }

    let outcome = state.notification_service.handle(&payload).await?;
    let body = if outcome.duplicate {
        NotifyResponse::duplicate(outcome.dedupe_key)
    } else {
        NotifyResponse::accepted(outcome.dedupe_key)
    };
    Ok((StatusCode::OK, Json(body)))
}
