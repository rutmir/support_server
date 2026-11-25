use crate::{api, config::Config, middleware, state::AppState};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

/// Build the full support-server router.
///
/// All `/api/v1/*` routes are protected by the bearer-key middleware; `/health`
/// stays open for liveness/readiness probes.
pub fn build_router(state: AppState) -> Router {
    let config: std::sync::Arc<Config> = state.config.clone();

    let api_router = Router::new()
        .route("/v1/error-report", post(api::error_report::report_error))
        .route("/v1/notify", post(api::notify::notify))
        .layer(axum::middleware::from_fn_with_state(
            config,
            middleware::auth::api_key_auth,
        ))
        .with_state(state);

    Router::new()
        .route("/health", get(|| async { "health" }))
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http())
}
