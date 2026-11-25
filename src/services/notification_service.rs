use crate::{
    cache::MessageCache, error::AppError, models::NotifyRequest, telegram::TelegramSender,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Outcome of processing one notification.
#[derive(Debug, Clone)]
pub struct NotifyOutcome {
    /// `true` when an identical notification was already sent within the TTL.
    pub duplicate: bool,
    /// Deterministic SHA-256 dedupe key (hex).
    pub dedupe_key: String,
}

/// Service that validates (already validated by the handler), deduplicates and
/// forwards notifications to Telegram through a [`TelegramSender`].
///
/// Dedupe is content-addressed: `app | severity | title | message | refs | url`
/// (values are trimmed/lower-cased where it does not change the meaning). The
/// key lives in the same moka TTL cache as the error-report dedupe
/// (`cache_ttl_minutes`), so two identical notifications inside the window are
/// delivered exactly once.
pub struct NotificationService {
    telegram: Arc<dyn TelegramSender>,
    cache: MessageCache,
}

impl NotificationService {
    pub fn new(telegram: Arc<dyn TelegramSender>, cache: MessageCache) -> Self {
        Self { telegram, cache }
    }

    /// Deterministic dedupe key for a notification.
    pub fn dedupe_key(request: &NotifyRequest) -> String {
        let mut hasher = Sha256::new();
        hasher.update(request.app.trim().to_lowercase().as_bytes());
        hasher.update([0u8]);
        hasher.update(request.severity.trim().to_lowercase().as_bytes());
        hasher.update([0u8]);
        hasher.update(request.title.trim().as_bytes());
        hasher.update([0u8]);
        hasher.update(request.message.trim().as_bytes());
        hasher.update([0u8]);
        for r in &request.refs {
            hasher.update(r.trim().as_bytes());
            hasher.update([0u8]);
        }
        if let Some(url) = request.url.as_deref() {
            hasher.update(url.trim().as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// Human-readable Telegram (HTML) message for a notification.
    pub fn format_message(request: &NotifyRequest) -> String {
        let mut out = String::new();
        out.push_str(&format!("<b>{}</b>\n", escape_html(&request.title)));
        out.push_str(&format!(
            "app: <code>{}</code> · severity: <b>{}</b>\n",
            escape_html(&request.app),
            escape_html(&request.severity)
        ));
        if let Some(source) = request.source.as_deref() {
            if !source.is_empty() {
                out.push_str(&format!("source: <code>{}</code>\n", escape_html(source)));
            }
        }
        if let Some(job_id) = request.job_id.as_deref() {
            if !job_id.is_empty() {
                out.push_str(&format!("job: <code>{}</code>\n", escape_html(job_id)));
            }
        }
        out.push('\n');
        out.push_str(&escape_html(&request.message));
        if let Some(url) = request.url.as_deref() {
            if !url.is_empty() {
                out.push_str(&format!("\n\nurl: {}", escape_html(url)));
            }
        }
        if !request.refs.is_empty() {
            out.push_str("\n\nrefs:");
            for r in &request.refs {
                out.push_str(&format!("\n• {}", escape_html(r)));
            }
        }
        out
    }

    /// Process a notification: dedupe check → Telegram send → cache insert.
    ///
    /// Errors from Telegram are propagated as `502 Bad Gateway`
    /// (`AppError::ExternalServiceError`); nothing is cached in that case so a
    /// later retry can still deliver.
    pub async fn handle(&self, request: &NotifyRequest) -> Result<NotifyOutcome, AppError> {
        let key = Self::dedupe_key(request);
        if self.cache.get(&key).await.is_some() {
            log::info!(
                "duplicate notification suppressed: app={} severity={} title={:?} key={}",
                request.app,
                request.severity,
                request.title,
                key
            );
            return Ok(NotifyOutcome {
                duplicate: true,
                dedupe_key: key,
            });
        }

        let message = Self::format_message(request);
        self.telegram.send(&message).await?;
        self.cache.insert(key.clone(), ()).await;
        log::info!(
            "notification delivered: app={} severity={} title={:?} job={:?} key={}",
            request.app,
            request.severity,
            request.title,
            request.job_id,
            key
        );
        Ok(NotifyOutcome {
            duplicate: false,
            dedupe_key: key,
        })
    }
}

/// Minimal HTML escape for Telegram `parse_mode=HTML`.
pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telegram::MockTelegramClient;

    fn req(title: &str) -> NotifyRequest {
        NotifyRequest {
            app: "argus".into(),
            severity: "warning".into(),
            title: title.into(),
            message: "job needs a human".into(),
            refs: vec!["job/code4rena-x".into()],
            url: None,
            source: Some("argus-dispatcher".into()),
            job_id: Some("code4rena-x".into()),
        }
    }

    #[tokio::test]
    async fn dedupe_key_is_stable_and_content_addressed() {
        let a = NotificationService::dedupe_key(&req("T"));
        let b = NotificationService::dedupe_key(&req("T"));
        let c = NotificationService::dedupe_key(&req("other"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[tokio::test]
    async fn handle_sends_once_and_suppresses_duplicate() {
        let mock = MockTelegramClient::new();
        let config = crate::config::Config::default();
        let service = NotificationService::new(std::sync::Arc::new(mock.clone()), crate::cache::init_cache(&config));

        let first = service.handle(&req("Needs human")).await.unwrap();
        assert!(!first.duplicate);
        assert_eq!(mock.count(), 1);

        let second = service.handle(&req("Needs human")).await.unwrap();
        assert!(second.duplicate);
        assert_eq!(mock.count(), 1, "duplicate must not hit Telegram again");
    }

    #[tokio::test]
    async fn format_message_escapes_html_and_contains_refs() {
        let mut r = req("<b>broken</b>");
        r.message = "a & b".into();
        let text = NotificationService::format_message(&r);
        assert!(text.contains("&lt;b&gt;broken&lt;/b&gt;"));
        assert!(text.contains("a &amp; b"));
        assert!(text.contains("job/code4rena-x"));
    }
}
