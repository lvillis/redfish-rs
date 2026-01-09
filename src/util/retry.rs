use std::time::{Duration, SystemTime};

use http::header::RETRY_AFTER;
use http::{HeaderMap, Method, StatusCode};

/// Retry policy for transient failures.
///
/// The policy is intentionally conservative: it only retries idempotent methods
/// by default.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    max_retries: usize,
    base_delay: Duration,
    max_delay: Duration,
    jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Maximum number of retries (not including the initial attempt).
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }

    /// Base delay for exponential backoff.
    pub fn base_delay(&self) -> Duration {
        self.base_delay
    }

    /// Maximum delay cap for exponential backoff.
    pub fn max_delay(&self) -> Duration {
        self.max_delay
    }

    /// Whether to apply jitter to backoff delays.
    pub fn jitter(&self) -> bool {
        self.jitter
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_base_delay(mut self, base_delay: Duration) -> Self {
        self.base_delay = base_delay;
        self
    }

    pub fn with_max_delay(mut self, max_delay: Duration) -> Self {
        self.max_delay = max_delay;
        self
    }

    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }
}

pub(crate) fn is_idempotent(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS
    )
}

pub(crate) fn should_retry_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

pub(crate) fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?;
    let s = value.to_str().ok()?.trim();
    if s.is_empty() {
        return None;
    }

    // 1) delta-seconds
    if let Ok(seconds) = s.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    // 2) HTTP-date
    if let Ok(when) = httpdate::parse_http_date(s) {
        let now = SystemTime::now();
        if let Ok(delta) = when.duration_since(now) {
            return Some(delta);
        }
        return Some(Duration::from_secs(0));
    }

    None
}

pub(crate) fn backoff_delay(policy: &RetryPolicy, attempt: usize) -> Duration {
    // attempt = 0 => base_delay
    let exp = 1u64 << (attempt.min(31) as u32);
    let base_ms = policy.base_delay.as_millis().min(u128::from(u64::MAX)) as u64;
    let mut delay_ms = base_ms.saturating_mul(exp);

    let max_ms = policy.max_delay.as_millis().min(u128::from(u64::MAX)) as u64;
    if delay_ms > max_ms {
        delay_ms = max_ms;
    }

    let mut delay = Duration::from_millis(delay_ms);

    if policy.jitter && delay_ms > 0 {
        // Full jitter: random value in [0, delay)
        let jitter_ms = fastrand::u64(0..delay_ms);
        delay = Duration::from_millis(jitter_ms);
    }

    delay
}
