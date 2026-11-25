//! Router-level tests for `POST /api/v1/notify` using an in-memory Telegram
//! mock (`MockTelegramClient`): auth, validation, delivery and dedupe.

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use support_server::{
    build_router,
    cache::init_cache,
    config::Config,
    services::error_report_service::ErrorReportService,
    services::notification_service::NotificationService,
    state::AppState,
    telegram::MockTelegramClient,
};
use tower::ServiceExt;

const TEST_KEY: &str = "test_api_key";

fn test_state(mock: MockTelegramClient) -> AppState {
    let mut config = Config::default();
    config.api_keys = vec![TEST_KEY.to_string()];
    let config = Arc::new(config);
    let cache = init_cache(&config);
    AppState {
        config: config.clone(),
        error_report_service: Arc::new(ErrorReportService::new(
            Arc::new(mock.clone()),
            cache.clone(),
        )),
        notification_service: Arc::new(NotificationService::new(Arc::new(mock), cache)),
    }
}

fn payload(title: &str, message: &str) -> Value {
    json!({
        "app": "argus",
        "severity": "warning",
        "title": title,
        "message": message,
        "refs": ["job/code4rena-2023-07-basin@73f7133b",
                 "http://192.168.88.48:8788/v1/jobs/code4rena-2023-07-basin-73f7133b380e"],
        "url": "http://192.168.88.48:8788/v1/jobs/code4rena-2023-07-basin-73f7133b380e",
        "source": "argus-dispatcher",
        "job_id": "code4rena-2023-07-basin-73f7133b380e"
    })
}

async fn post_notify(app: Router, body: Value, token: Option<&str>) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/notify")
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let response = app
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1 << 20)
        .await
        .unwrap();
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

#[tokio::test]
async fn notify_requires_bearer_key() {
    let mock = MockTelegramClient::new();
    let app = build_router(test_state(mock.clone()));

    let (status, _) = post_notify(app, payload("t", "m"), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn notify_rejects_wrong_bearer_key() {
    let mock = MockTelegramClient::new();
    let app = build_router(test_state(mock.clone()));

    let (status, _) = post_notify(app, payload("t", "m"), Some("wrong-key")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn notify_accepted_then_deduped() {
    let mock = MockTelegramClient::new();
    let app = build_router(test_state(mock.clone()));
    let body = payload("Needs human review", "job finished triage with 3 needs_human");

    let (status, first) = post_notify(app.clone(), body.clone(), Some(TEST_KEY)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(first["status"], "success");
    assert_eq!(first["duplicate"], false);
    assert_eq!(mock.count(), 1, "first notification delivered");
    let first_key = first["dedupe_key"].as_str().unwrap().to_string();
    assert_eq!(first_key.len(), 64);

    let (status, second) = post_notify(app, body, Some(TEST_KEY)).await;
    assert_eq!(status, StatusCode::OK, "dedupe is an idempotent 200");
    assert_eq!(second["duplicate"], true);
    assert_eq!(second["dedupe_key"], first_key);
    assert_eq!(mock.count(), 1, "duplicate must not hit Telegram");

    // Telegram got the HTML-formatted human-readable message.
    let sent = mock.sent();
    assert!(sent[0].contains("Needs human review"));
    assert!(sent[0].contains("code4rena-2023-07-basin"));
}

#[tokio::test]
async fn notify_validates_required_fields() {
    let mock = MockTelegramClient::new();
    let app = build_router(test_state(mock.clone()));

    let (status, body) = post_notify(
        app,
        json!({"app":"argus","severity":"warning","title":"","message":"m"}),
        Some(TEST_KEY),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Bad request");
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn notify_telegram_failure_is_502() {
    let mock = MockTelegramClient::failing();
    let app = build_router(test_state(mock.clone()));

    let (status, _) = post_notify(app, payload("t", "m"), Some(TEST_KEY)).await;
    assert_eq!(status, StatusCode::BAD_GATEWAY);
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn notify_distinct_messages_are_both_delivered() {
    let mock = MockTelegramClient::new();
    let app = build_router(test_state(mock.clone()));

    let (s1, _) = post_notify(app.clone(), payload("a", "one"), Some(TEST_KEY)).await;
    let (s2, _) = post_notify(app, payload("b", "two"), Some(TEST_KEY)).await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(mock.count(), 2);
}

#[tokio::test]
async fn health_endpoint_is_open() {
    let app = build_router(test_state(MockTelegramClient::new()));
    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
