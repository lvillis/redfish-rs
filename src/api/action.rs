use std::time::Duration;

use http::StatusCode;
use http::header::LOCATION;
use serde::de::DeserializeOwned;
use url::Url;

use crate::transport::RawResponse;
use crate::{Error, RequestContext, Result};

/// A generic helper for endpoints that return a status + optional `Location` + optional JSON body.
///
/// This is commonly used by Redfish Actions (e.g. `ComputerSystem.Reset`) and create operations
/// that return `201 Created` with a `Location` header.
#[derive(Debug, Clone)]
pub struct ActionResponse<T = serde_json::Value> {
    pub status: StatusCode,
    pub location: Option<String>,
    pub retry_after: Option<Duration>,
    pub body: Option<T>,
}

impl<T> ActionResponse<T> {
    pub(crate) fn from_raw_json(
        method: http::Method,
        url: &Url,
        raw: RawResponse,
        body_snippet_limit: usize,
    ) -> Result<Self>
    where
        T: DeserializeOwned,
    {
        let location = raw
            .headers
            .get(LOCATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);

        let retry_after = crate::util::retry::parse_retry_after(&raw.headers);

        let body = if raw.body.is_empty() {
            None
        } else {
            let ctx = RequestContext::new(method, url.to_string());
            serde_json::from_slice::<T>(&raw.body)
                .map(Some)
                .map_err(|e| {
                    let snippet =
                        crate::util::redact::redacted_body_snippet(&raw.body, body_snippet_limit);
                    Error::decode(
                        ctx,
                        format!("Failed to decode JSON response: {e}"),
                        snippet,
                        e,
                    )
                })?
        };

        Ok(Self {
            status: raw.status,
            location,
            retry_after,
            body,
        })
    }
}
