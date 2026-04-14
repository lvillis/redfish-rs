use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

use http::{Method, StatusCode};

/// Coarse error category. Use [`Error::kind`] to inspect.
///
/// This enum is marked `non_exhaustive` for forward compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Configuration/validation error (bad base URL, invalid header value, etc).
    InvalidConfig,
    /// HTTP transport error (I/O, TLS, DNS, etc).
    Transport,
    /// Request timed out.
    Timeout,
    /// Response body could not be parsed/decoded.
    Decode,
    /// Non-success HTTP status from the Redfish service.
    Api,
    /// Auth-related error (missing token, unauthorized, etc).
    Auth,
    /// Client-side cancellation (if supported by the runtime / transport).
    Canceled,
}

/// Additional context about the request that triggered an error.
#[derive(Debug, Clone)]
pub struct RequestContext {
    method: Method,
    url: String,
}

impl RequestContext {
    pub(crate) fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
        }
    }

    /// HTTP method used for the request.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Full request URL.
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// Error type returned by this crate.
///
/// Design goals:
///
/// - Preserve actionable context (method, URL, status, request-id)
/// - Avoid leaking secrets (auth headers, passwords, session tokens)
/// - Do not expose backend-specific HTTP client types in the public API
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Box<ErrorInner>,
}

#[derive(Debug)]
struct ErrorInner {
    message: String,
    context: Option<RequestContext>,
    status: Option<StatusCode>,
    request_id: Option<String>,
    retry_after: Option<Duration>,
    body_snippet: Option<String>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl Error {
    pub(crate) fn invalid_config(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::InvalidConfig,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: None,
                status: None,
                request_id: None,
                retry_after: None,
                body_snippet: None,
                source: None,
            }),
        }
    }

    pub(crate) fn auth(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Auth,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: None,
                status: None,
                request_id: None,
                retry_after: None,
                body_snippet: None,
                source: None,
            }),
        }
    }

    pub(crate) fn auth_http(
        context: RequestContext,
        status: StatusCode,
        message: impl Into<String>,
        request_id: Option<String>,
        body_snippet: Option<String>,
    ) -> Self {
        Self {
            kind: ErrorKind::Auth,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: Some(context),
                status: Some(status),
                request_id,
                retry_after: None,
                body_snippet,
                source: None,
            }),
        }
    }

    pub(crate) fn transport(
        context: RequestContext,
        message: impl Into<String>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Transport,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: Some(context),
                status: None,
                request_id: None,
                retry_after: None,
                body_snippet: None,
                source: Some(Box::new(source)),
            }),
        }
    }

    pub(crate) fn timeout(
        context: RequestContext,
        message: impl Into<String>,
        source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            kind: ErrorKind::Timeout,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: Some(context),
                status: None,
                request_id: None,
                retry_after: None,
                body_snippet: None,
                source,
            }),
        }
    }

    pub(crate) fn decode(
        context: RequestContext,
        message: impl Into<String>,
        body_snippet: Option<String>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Decode,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: Some(context),
                status: None,
                request_id: None,
                retry_after: None,
                body_snippet,
                source: Some(Box::new(source)),
            }),
        }
    }

    pub(crate) fn api(
        context: RequestContext,
        status: StatusCode,
        message: impl Into<String>,
        request_id: Option<String>,
        retry_after: Option<Duration>,
        body_snippet: Option<String>,
    ) -> Self {
        Self {
            kind: ErrorKind::Api,
            inner: Box::new(ErrorInner {
                message: message.into(),
                context: Some(context),
                status: Some(status),
                request_id,
                retry_after,
                body_snippet,
                source: None,
            }),
        }
    }

    /// Get the high-level error category.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// If the error is caused by a non-success HTTP status, return that status.
    pub fn status(&self) -> Option<StatusCode> {
        self.inner.status
    }

    /// If present, return the request-id/correlation-id from the server response.
    pub fn request_id(&self) -> Option<&str> {
        self.inner.request_id.as_deref()
    }

    /// If the server returned a `Retry-After` hint, return it.
    pub fn retry_after(&self) -> Option<Duration> {
        self.inner.retry_after
    }

    /// A redacted snippet of the response body, useful for debugging.
    ///
    /// This is intentionally truncated and may be absent depending on configuration.
    pub fn body_snippet(&self) -> Option<&str> {
        self.inner.body_snippet.as_deref()
    }

    /// Context for the request that failed.
    pub fn context(&self) -> Option<&RequestContext> {
        self.inner.context.as_ref()
    }

    /// Whether this error is typically safe to retry.
    pub fn is_retryable(&self) -> bool {
        match self.kind {
            ErrorKind::Timeout | ErrorKind::Transport => true,
            ErrorKind::Api => matches!(
                self.inner.status,
                Some(StatusCode::TOO_MANY_REQUESTS)
                    | Some(StatusCode::BAD_GATEWAY)
                    | Some(StatusCode::SERVICE_UNAVAILABLE)
                    | Some(StatusCode::GATEWAY_TIMEOUT)
            ),
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.inner.context, self.inner.status) {
            (Some(ctx), Some(status)) => write!(
                f,
                "{} (status {}, {} {})",
                self.inner.message,
                status.as_u16(),
                ctx.method.as_str(),
                ctx.url
            )?,
            (Some(ctx), None) => write!(
                f,
                "{} ({} {})",
                self.inner.message,
                ctx.method.as_str(),
                ctx.url
            )?,
            (None, Some(status)) => {
                write!(f, "{} (status {})", self.inner.message, status.as_u16())?
            }
            (None, None) => write!(f, "{}", self.inner.message)?,
        }

        if let Some(id) = &self.inner.request_id {
            write!(f, ", request-id={id}")?;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .source
            .as_deref()
            .map(|err| err as &(dyn StdError + 'static))
    }
}
