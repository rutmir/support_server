use support_server::{
    cache::init_cache,
    config::Config,
    models::error_report::{App, Device, Error, ErrorReportRequest},
    services::error_report_service::ErrorReportService,
    telegram::MockTelegramClient,
};
use std::sync::Arc;

fn request(message: &str) -> ErrorReportRequest {
    ErrorReportRequest {
        app: App {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
        },
        device: Device {
            id: "device-123".to_string(),
            description: Some("Test Device".to_string()),
        },
        error: Error {
            message: message.to_string(),
            trace: vec![
                "stack trace line 1".to_string(),
                "stack trace line 2".to_string(),
            ],
        },
    }
}

#[tokio::test]
async fn test_successful_report_handling() {
    let config = Config::default();
    let message_cache = init_cache(&config);
    let mock = MockTelegramClient::new();
    let service = ErrorReportService::new(Arc::new(mock.clone()), message_cache);

    let result = service.handle_report(request("Test error message")).await;
    assert!(result.is_ok());
    assert_eq!(mock.count(), 1, "non-duplicate report must reach Telegram");
}

#[tokio::test]
async fn test_duplicate_detection() {
    let config = Config::default();
    let message_cache = init_cache(&config);
    let mock = MockTelegramClient::new();
    let service = ErrorReportService::new(Arc::new(mock.clone()), message_cache);

    let first = service.handle_report(request("Duplicate me")).await;
    let second = service.handle_report(request("Duplicate me")).await;

    assert!(first.is_ok());
    assert!(second.is_ok(), "duplicate is silently ignored (Ok)");
    assert_eq!(mock.count(), 1, "duplicate must not hit Telegram");
}

#[tokio::test]
async fn test_different_error_is_not_duplicate() {
    let config = Config::default();
    let message_cache = init_cache(&config);
    let mock = MockTelegramClient::new();
    let service = ErrorReportService::new(Arc::new(mock.clone()), message_cache);

    service.handle_report(request("error one")).await.unwrap();
    service.handle_report(request("error two")).await.unwrap();
    assert_eq!(mock.count(), 2);
}

#[tokio::test]
async fn test_generate_message_hash_is_stable() {
    let config = Config::default();
    let message_cache = init_cache(&config);
    let service = ErrorReportService::new(Arc::new(MockTelegramClient::new()), message_cache);

    let a = service.generate_message_hash(&request("same"));
    let b = service.generate_message_hash(&request("same"));
    let c = service.generate_message_hash(&request("other"));
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_eq!(a.len(), 64);
}
