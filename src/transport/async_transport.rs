use std::time::Duration;

use http::Method;
use url::Url;

use crate::transport::{
    RawResponse, Request, classify_http_error, retry_after_from_response, should_retry_response,
};
use crate::util::retry::backoff_delay;
use crate::{Error, RequestContext};

#[cfg(feature = "tracing")]
use tracing::{debug, warn};

/// Async transport implementation (reqwest + tokio).
#[derive(Clone)]
pub(crate) struct AsyncTransport {
    client: reqwest::Client,
    retry: crate::util::retry::RetryPolicy,
    body_snippet_limit: usize,
}

impl AsyncTransport {
    pub(crate) fn new(
        client: reqwest::Client,
        retry: crate::util::retry::RetryPolicy,
        body_snippet_limit: usize,
    ) -> Self {
        Self {
            client,
            retry,
            body_snippet_limit,
        }
    }

    pub(crate) async fn send(&self, req: Request) -> Result<RawResponse, Error> {
        let mut attempt: usize = 0;

        loop {
            let result = self.send_once(&req).await;

            match result {
                Ok(resp) => {
                    if resp.status.is_success() {
                        return Ok(resp);
                    }

                    if attempt < self.retry.max_retries()
                        && should_retry_response(&req.method, &resp)
                    {
                        let delay = retry_after_from_response(&resp)
                            .unwrap_or_else(|| backoff_delay(&self.retry, attempt));

                        #[cfg(feature = "tracing")]
                        warn!(
                            method = %req.method,
                            url = %req.url,
                            status = %resp.status.as_u16(),
                            attempt,
                            ?delay,
                            "redfish request retrying due to HTTP status"
                        );

                        sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    return Err(classify_http_error(
                        &req.method,
                        &req.url,
                        &resp,
                        self.body_snippet_limit,
                    ));
                }
                Err(err) => {
                    if attempt < self.retry.max_retries()
                        && crate::util::retry::is_idempotent(&req.method)
                        && err.is_retryable()
                    {
                        let delay = backoff_delay(&self.retry, attempt);

                        #[cfg(feature = "tracing")]
                        warn!(
                            method = %req.method,
                            url = %req.url,
                            attempt,
                            ?delay,
                            error = %err,
                            "redfish request retrying due to transport error"
                        );

                        sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    return Err(err);
                }
            }
        }
    }

    async fn send_once(&self, req: &Request) -> Result<RawResponse, Error> {
        #[cfg(feature = "tracing")]
        debug!(method = %req.method, url = %req.url, "redfish request sending");

        let mut builder = self
            .client
            .request(req.method.clone(), req.url.clone())
            .headers(req.headers.clone());

        if let Some(body) = req.body.clone() {
            builder = builder.body(body);
        }

        let response = builder
            .send()
            .await
            .map_err(|e| map_reqwest_error(&req.method, &req.url, e))?;

        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .bytes()
            .await
            .map_err(|e| map_reqwest_error(&req.method, &req.url, e))?
            .to_vec();

        Ok(RawResponse {
            status,
            headers,
            body,
        })
    }
}

fn map_reqwest_error(method: &Method, url: &Url, err: reqwest::Error) -> Error {
    let ctx = RequestContext::new(method.clone(), url.to_string());

    if err.is_timeout() {
        return Error::timeout(ctx, "Request timed out", Some(Box::new(err)));
    }

    // reqwest::Error exposes `is_request`/`is_connect`/`is_body` etc, but we keep
    // a coarse classification at this layer.
    Error::transport(ctx, "Transport error", err)
}

async fn sleep(delay: Duration) {
    if delay.is_zero() {
        return;
    }
    tokio::time::sleep(delay).await;
}
