use std::sync::Arc;

use crate::Error;
use crate::transport::{RawResponse, Request, classify_http_error, map_reqx_error};

/// Blocking transport implementation (`reqx::blocking`).
#[derive(Clone)]
pub(crate) struct BlockingTransport {
    client: Arc<reqx::blocking::Client>,
    body_snippet_limit: usize,
}

impl BlockingTransport {
    pub(crate) fn new(client: reqx::blocking::Client, body_snippet_limit: usize) -> Self {
        Self {
            client: Arc::new(client),
            body_snippet_limit,
        }
    }

    pub(crate) fn send(&self, req: Request) -> Result<RawResponse, Error> {
        let response = self.send_once(&req)?;
        if response.status.is_success() {
            Ok(response)
        } else {
            Err(classify_http_error(
                &req.method,
                &req.url,
                &response,
                self.body_snippet_limit,
            ))
        }
    }

    fn send_once(&self, req: &Request) -> Result<RawResponse, Error> {
        let mut builder = self.client.request(req.method.clone(), req.url.as_str());

        for (name, value) in &req.headers {
            builder = builder.header(name.clone(), value.clone());
        }

        if let Some(body) = req.body.clone() {
            builder = builder.body(body);
        }

        let response = builder
            .send_response()
            .map_err(|e| map_reqx_error(&req.method, &req.url, e))?;

        let status = response.status();
        let headers = response.headers().clone();
        let body = response.body().to_vec();

        Ok(RawResponse {
            status,
            headers,
            body,
        })
    }
}
