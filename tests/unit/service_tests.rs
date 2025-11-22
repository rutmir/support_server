use support_server::{
    services::error_report_service::ErrorReportService,
    models::error_report::{ErrorReportRequest, Device, Error},
    telegram::TelegramClient,
    cache::init_cache,
    config::Config,
};
use std::time::Duration;

#[tokio::test]
async fn test_successful_report_handling() {
    // Create a cache with default configuration
    let config = Config::default();
    let message_cache = init_cache(&config);
    
    // Create a Telegram client with empty configuration (will skip sending)
    let telegram_config = Config {
        telegram_bot_token: String::new(),
        telegram_chat_id: String::new(),
        ..Config::default()
    };
    let telegram_client = TelegramClient::new(&telegram_config);
    
    // Create the service
    let service = ErrorReportService::new(telegram_client, message_cache);
    
    // Create a test request
    let request = ErrorReportRequest {
        device: Device {
            id: "device-123".to_string(),
            description: Some("Test Device".to_string()),
        },
        error: Error {
            message: "Test error message".to_string(),
            trace: vec!["stack trace line 1".to_string(), "stack trace line 2".to_string()],
        },
    };
    
    // Handle the report
    let result = service.handle_report(request).await;
    
    // Assert that the result is Ok
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_duplicate_detection() {
    // Create a cache with default configuration
    let config = Config::default();
    let message_cache = init_cache(&config);
    
    // Create a Telegram client with empty configuration (will skip sending)
    let telegram_config = Config {
        telegram_bot_token: String::new(),
        telegram_chat_id: String::new(),
        ..Config::default()
    };
    let telegram_client = TelegramClient::new(&telegram_config);
    
    // Create the service
    let service = ErrorReportService::new(telegram_client, message_cache);
    
    // Create a test request
    let request = ErrorReportRequest {
        device: Device {
            id: "device-456".to_string(),
            description: Some("Test Device 2".to_string()),
        },
        error: Error {
            message: "Duplicate error message".to_string(),
            trace: vec!["stack trace line 1".to_string(), "stack trace line 2".to_string()],
        },
    };
    
    // Handle the report for the first time
    let result1 = service.handle_report(request.clone()).await;
    assert!(result1.is_ok());
    
    // Handle the report for the second time (should be detected as duplicate)
    let result2 = service.handle_report(request).await;
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_hashing_consistency() {
    // Create a cache with default configuration
    let config = Config::default();
    let message_cache = init_cache(&config);
    
    // Create a Telegram client with empty configuration (will skip sending)
    let telegram_config = Config {
        telegram_bot_token: String::new(),
        telegram_chat_id: String::new(),
        ..Config::default()
    };
    let telegram_client = TelegramClient::new(&telegram_config);
    
    // Create the service
    let service = ErrorReportService::new(telegram_client, message_cache);
    
    // Create a test request
    let request = ErrorReportRequest {
        device: Device {
            id: "device-789".to_string(),
            description: Some("Test Device 3".to_string()),
        },
        error: Error {
            message: "Consistency test error".to_string(),
            trace: vec!["stack trace line 1".to_string(), "stack trace line 2".to_string()],
        },
    };
    
    // Generate hash twice
    let hash1 = service.generate_message_hash(&request);
    let hash2 = service.generate_message_hash(&request);
    
    // Assert that the hashes are identical
    assert_eq!(hash1, hash2);
}