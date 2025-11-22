use axum::{
    routing::{get, post},
    Router,
    serve,
};
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use env_logger;
use tower_http::trace::TraceLayer;

use crate::services::error_report_service::ErrorReportService;

mod api;
mod cache;
mod config;
mod error;
mod middleware;
mod services;
mod state;
mod telegram;
mod models;

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
    
    // Initialize message cache with TTL from config
    let message_cache = cache::init_cache(&config);
    log::info!("Message cache initialized with {} minute TTL", config.cache_ttl_minutes);
    
    // Initialize Telegram client
    let telegram_client = telegram::TelegramClient::new(&config);
    log::info!("Telegram client initialized");
    
    // Initialize Error Report Service
    let error_report_service = Arc::new(ErrorReportService::new(telegram_client, message_cache));
    log::info!("Error report service initialized");
    
    // Create application state
    let app_state = state::AppState {
        config: config.clone(),
        error_report_service,
    };
    
    // Build our application with the API key auth middleware
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) // Placeholder route
        .route("/api/v1/error-report", post(api::error_report::report_error))
        .layer(axum::middleware::from_fn_with_state(
            config.clone(),
            middleware::auth::api_key_auth,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    // Run our app with hyper, listening globally on port 3000
    let addr: SocketAddr = config.server_address.parse().expect("Unable to parse socket address");
    log::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
