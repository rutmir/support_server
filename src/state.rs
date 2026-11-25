use crate::{config::Config, services::error_report_service::ErrorReportService, services::notification_service::NotificationService};
use std::sync::Arc;

/// Application state that holds shared resources
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub error_report_service: Arc<ErrorReportService>,
    pub notification_service: Arc<NotificationService>,
}
