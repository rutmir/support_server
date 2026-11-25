use std::{net::SocketAddr, process, sync::Arc};
use support_server::{
    build_router,
    cache,
    config,
    services::error_report_service::ErrorReportService,
    services::notification_service::NotificationService,
    state::AppState,
    telegram::TelegramClient,
};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = match config::Config::from_env_and_file() {
        Ok(config) => {
            log::info!("Configuration loaded successfully");
            Arc::new(config)
        }
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    log::info!("Starting support server on {}", config.server_address);

    // Initialize message cache with TTL from config (shared by both services)
    let message_cache = cache::init_cache(&config);
    log::info!("Message cache initialized with {} minute TTL", config.cache_ttl_minutes);

    // Initialize Telegram client (base URL is overridable for local e2e tests)
    let telegram_client = Arc::new(TelegramClient::new(&config));
    log::info!("Telegram client initialized");

    // Initialize Error Report Service
    let error_report_service = Arc::new(ErrorReportService::new(
        telegram_client.clone(),
        message_cache.clone(),
    ));
    log::info!("Error report service initialized");

    // Initialize Notification gateway Service (/api/v1/notify)
    let notification_service = Arc::new(NotificationService::new(
        telegram_client,
        message_cache,
    ));
    log::info!("Notification service initialized");

    // Create application state
    let app_state = AppState {
        config: config.clone(),
        error_report_service,
        notification_service,
    };

    let app = build_router(app_state);

    let addr: SocketAddr = config
        .server_address
        .parse()
        .expect("Unable to parse socket address");
    log::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
