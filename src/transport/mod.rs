use http::{HeaderMap, Method, StatusCode};
use serde::Deserialize;
use url::Url;

use crate::util::redact::redacted_body_snippet;
use crate::util::retry::{parse_retry_after, should_retry_status};
use crate::{Error, RequestContext};

#[cfg(feature = "async")]
mod async_transport;
#[cfg(feature = "blocking")]
mod blocking_transport;

#[cfg(feature = "async")]
pub(crate) use async_transport::AsyncTransport;
#[cfg(feature = "blocking")]
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

pub(crate) fn should_retry_response(method: &Method, response: &RawResponse) -> bool {
    crate::util::retry::is_idempotent(method) && should_retry_status(response.status)
}

pub(crate) fn retry_after_from_response(response: &RawResponse) -> Option<std::time::Duration> {
    parse_retry_after(&response.headers)
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
