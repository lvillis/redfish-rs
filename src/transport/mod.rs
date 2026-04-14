use std::time::Duration;

use http::{HeaderMap, Method, StatusCode};
use serde::Deserialize;
use url::Url;

use crate::util::redact::redacted_body_snippet;
use crate::util::retry::{RetryPolicy, backoff_delay, parse_retry_after};
use crate::{Error, RequestContext};

#[cfg(feature = "_async")]
mod async_transport;
#[cfg(feature = "_blocking")]
mod blocking_transport;

#[cfg(feature = "_async")]
pub(crate) use async_transport::AsyncTransport;
#[cfg(feature = "_blocking")]
pub(crate) use blocking_transport::BlockingTransport;

/// Internal request representation.
#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderMap,
    pub body: Option<Vec<u8>>,
}

/// Raw response (status, headers, body bytes).
#[derive(Debug, Clone)]
pub(crate) struct RawResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

#[cfg(feature = "_async")]
pub(crate) fn default_async_tls_backend() -> reqx::TlsBackend {
    #[cfg(feature = "async-tls-rustls-ring")]
    {
        return reqx::TlsBackend::RustlsRing;
    }

    #[cfg(all(
        not(feature = "async-tls-rustls-ring"),
        feature = "async-tls-rustls-aws-lc-rs"
    ))]
    {
        return reqx::TlsBackend::RustlsAwsLcRs;
    }

    #[cfg(all(
        not(feature = "async-tls-rustls-ring"),
        not(feature = "async-tls-rustls-aws-lc-rs"),
        feature = "async-tls-native"
    ))]
    {
        return reqx::TlsBackend::NativeTls;
    }

    #[allow(unreachable_code)]
    reqx::TlsBackend::RustlsRing
}

#[cfg(feature = "_blocking")]
pub(crate) fn default_blocking_tls_backend() -> reqx::TlsBackend {
    #[cfg(feature = "blocking-tls-rustls-ring")]
    {
        return reqx::TlsBackend::RustlsRing;
    }

    #[cfg(all(
        not(feature = "blocking-tls-rustls-ring"),
        feature = "blocking-tls-rustls-aws-lc-rs"
    ))]
    {
        return reqx::TlsBackend::RustlsAwsLcRs;
    }

    #[cfg(all(
        not(feature = "blocking-tls-rustls-ring"),
        not(feature = "blocking-tls-rustls-aws-lc-rs"),
        feature = "blocking-tls-native"
    ))]
    {
        return reqx::TlsBackend::NativeTls;
    }

    #[allow(unreachable_code)]
    reqx::TlsBackend::RustlsRing
}

#[derive(Clone, Debug)]
pub(crate) struct ReqxBackoffSource {
    policy: RetryPolicy,
}

impl reqx::advanced::BackoffSource for ReqxBackoffSource {
    fn backoff_for_retry(
        &self,
        _retry_policy: &reqx::prelude::RetryPolicy,
        attempt: usize,
    ) -> Duration {
        backoff_delay(&self.policy, attempt.saturating_sub(1))
    }
}

pub(crate) fn reqx_backoff_source(policy: RetryPolicy) -> ReqxBackoffSource {
    ReqxBackoffSource { policy }
}

pub(crate) fn reqx_retry_policy(policy: &RetryPolicy) -> reqx::prelude::RetryPolicy {
    let base_backoff = policy.base_delay().max(Duration::from_millis(1));
    let max_backoff = policy.max_delay().max(base_backoff);

    reqx::prelude::RetryPolicy::standard()
        .max_attempts(policy.max_retries().saturating_add(1))
        .base_backoff(base_backoff)
        .max_backoff(max_backoff)
        .jitter_ratio(0.0)
        .retryable_status_codes([
            StatusCode::TOO_MANY_REQUESTS.as_u16(),
            StatusCode::BAD_GATEWAY.as_u16(),
            StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            StatusCode::GATEWAY_TIMEOUT.as_u16(),
        ])
}

pub(crate) fn classify_http_error(
    method: &Method,
    url: &Url,
    response: &RawResponse,
    body_snippet_limit: usize,
) -> Error {
    let ctx = RequestContext::new(method.clone(), url.to_string());
    let request_id = extract_request_id(&response.headers);
    let retry_after = parse_retry_after(&response.headers);

    let body_snippet = redacted_body_snippet(&response.body, body_snippet_limit);

    // Prefer Redfish error payload when available.
    let message = extract_redfish_error_message(&response.body)
        .unwrap_or_else(|| format!("Redfish API returned HTTP {}", response.status.as_u16()));

    if matches!(
        response.status,
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN
    ) {
        return Error::auth_http(ctx, response.status, message, request_id, body_snippet);
    }

    Error::api(
        ctx,
        response.status,
        message,
        request_id,
        retry_after,
        body_snippet,
    )
}

pub(crate) fn map_reqx_error(method: &Method, url: &Url, err: reqx::Error) -> Error {
    let ctx = RequestContext::new(method.clone(), url.to_string());

    match err {
        reqx::Error::HttpStatus {
            status,
            headers,
            body,
            ..
        } => {
            let response = RawResponse {
                status: StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                headers: *headers,
                body: body.into_bytes(),
            };
            classify_http_error(method, url, &response, 8 * 1024)
        }
        reqx::Error::Timeout { .. } | reqx::Error::DeadlineExceeded { .. } => {
            Error::timeout(ctx, "Request timed out", Some(Box::new(err)))
        }
        reqx::Error::InvalidUri { .. }
        | reqx::Error::InvalidNoProxyRule { .. }
        | reqx::Error::InvalidProxyConfig { .. }
        | reqx::Error::ProxyAuthorizationRequiresHttpProxy
        | reqx::Error::InvalidAdaptiveConcurrencyPolicy { .. }
        | reqx::Error::RequestBuild { .. }
        | reqx::Error::InvalidHeaderName { .. }
        | reqx::Error::InvalidHeaderValue { .. }
        | reqx::Error::TlsBackendUnavailable { .. }
        | reqx::Error::TlsBackendInit { .. }
        | reqx::Error::TlsConfig { .. } => {
            Error::invalid_config(format!("HTTP client configuration error: {err}"))
        }
        _ => Error::transport(ctx, "Transport error", err),
    }
}

#[cfg(feature = "tracing")]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ReqxTracingObserver;

#[cfg(feature = "tracing")]
impl reqx::advanced::Observer for ReqxTracingObserver {
    fn on_request_start(&self, context: &reqx::advanced::RequestContext) {
        tracing::debug!(
            method = %context.method(),
            url = %context.uri(),
            attempt = context.attempt(),
            max_attempts = context.max_attempts(),
            "redfish request sending"
        );
    }

    fn on_retry_scheduled(
        &self,
        context: &reqx::advanced::RequestContext,
        decision: &reqx::advanced::RetryDecision,
        delay: Duration,
    ) {
        if let Some(status) = decision.status() {
            tracing::warn!(
                method = %context.method(),
                url = %context.uri(),
                status = status.as_u16(),
                attempt = decision.attempt(),
                ?delay,
                "redfish request retrying due to HTTP status"
            );
            return;
        }

        if let Some(phase) = decision.timeout_phase() {
            tracing::warn!(
                method = %context.method(),
                url = %context.uri(),
                attempt = decision.attempt(),
                timeout_phase = %phase,
                ?delay,
                "redfish request retrying due to transport timeout"
            );
            return;
        }

        if let Some(kind) = decision.transport_error_kind() {
            tracing::warn!(
                method = %context.method(),
                url = %context.uri(),
                attempt = decision.attempt(),
                transport_kind = %kind,
                ?delay,
                "redfish request retrying due to transport error"
            );
            return;
        }

        if decision.is_response_body_read_error() {
            tracing::warn!(
                method = %context.method(),
                url = %context.uri(),
                attempt = decision.attempt(),
                ?delay,
                "redfish request retrying due to response body read error"
            );
        }
    }
}

pub(crate) fn extract_request_id(headers: &HeaderMap) -> Option<String> {
    const CANDIDATES: [&str; 6] = [
        "x-request-id",
        "x-requestid",
        "x-correlation-id",
        "x-correlationid",
        "request-id",
        "trace-id",
    ];

    for name in CANDIDATES {
        if let Ok(h) = http::header::HeaderName::from_bytes(name.as_bytes())
            && let Some(v) = headers.get(h)
            && let Ok(s) = v.to_str()
        {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }

    None
}

#[derive(Debug, Deserialize)]
struct RedfishErrorEnvelope {
    #[serde(default)]
    error: Option<RedfishErrorBody>,
}

#[derive(Debug, Deserialize)]
struct RedfishErrorBody {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    message: Option<String>,

    #[serde(rename = "@Message.ExtendedInfo", default)]
    extended_info: Option<Vec<RedfishExtendedInfo>>,
}

#[derive(Debug, Deserialize)]
struct RedfishExtendedInfo {
    #[serde(rename = "Message", default)]
    message: Option<String>,

    #[serde(rename = "MessageId", default)]
    message_id: Option<String>,

    #[serde(rename = "Resolution", default)]
    resolution: Option<String>,
}

pub(crate) fn extract_redfish_error_message(body: &[u8]) -> Option<String> {
    let env = serde_json::from_slice::<RedfishErrorEnvelope>(body).ok()?;
    let err = env.error?;

    let mut parts: Vec<String> = Vec::new();

    if let Some(code) = err.code
        && !code.trim().is_empty()
    {
        parts.push(code);
    }
    if let Some(message) = err.message {
        let m = message.trim();
        if !m.is_empty() {
            parts.push(m.to_string());
        }
    }

    if let Some(info) = err.extended_info.and_then(|mut v| v.drain(..).next()) {
        if let Some(mid) = info.message_id {
            let mid = mid.trim();
            if !mid.is_empty() {
                parts.push(format!("message-id={mid}"));
            }
        }
        if let Some(msg) = info.message {
            let msg = msg.trim();
            if !msg.is_empty() {
                parts.push(msg.to_string());
            }
        }
        if let Some(res) = info.resolution {
            let res = res.trim();
            if !res.is_empty() {
                parts.push(format!("resolution={res}"));
            }
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(": "))
    }
}
