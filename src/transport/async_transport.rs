use crate::Error;
use crate::transport::{RawResponse, Request, classify_http_error, map_reqx_error};

/// Async transport implementation (`reqx` + tokio).
#[derive(Clone)]
pub(crate) struct AsyncTransport {
    client: reqx::Client,
    body_snippet_limit: usize,
}

impl AsyncTransport {
    pub(crate) fn new(client: reqx::Client, body_snippet_limit: usize) -> Self {
        Self {
            client,
            body_snippet_limit,
        }
    }

    pub(crate) async fn send(&self, req: Request) -> Result<RawResponse, Error> {
        let response = self.send_once(&req).await?;
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

    async fn send_once(&self, req: &Request) -> Result<RawResponse, Error> {
        let mut builder = self.client.request(req.method.clone(), req.url.as_str());

        for (name, value) in &req.headers {
            builder = builder.header(name.clone(), value.clone());
        }

        if let Some(body) = req.body.clone() {
            builder = builder.body(body);
        }

        let response = builder
            .send_response()
            .await
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
