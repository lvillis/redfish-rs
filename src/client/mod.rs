use std::time::Duration;

use http::header::{ACCEPT, CONTENT_TYPE};
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::{
    AccountServiceService, ChassisResourceService, ChassisService, EventServiceService,
    JsonSchemasService, ManagerResourceService, ManagersService, RegistriesService,
    ServiceRootService, SessionsService, SystemResourceService, SystemsService, TaskServiceService,
    UpdateServiceService,
};
use crate::transport::{RawResponse, Request};
use crate::util::retry::RetryPolicy;
use crate::util::url::{normalize_base_url, resolve_uri};
use crate::{Auth, Error, RequestContext, Result};

const DEFAULT_BODY_SNIPPET_LIMIT: usize = 8 * 1024;

#[cfg(feature = "rustls")]
fn ensure_rustls_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

#[cfg(not(feature = "rustls"))]
fn ensure_rustls_provider() {}

/// Shared, runtime-agnostic client configuration.
#[derive(Clone)]
struct ClientBase {
    base_url: Url,
    redfish_root: Vec<String>,
    auth: Auth,
    default_headers: HeaderMap,
    body_snippet_limit: usize,
}

impl ClientBase {
    fn redfish_url(&self, extra_segments: &[&str]) -> Result<Url> {
        let mut segments: Vec<&str> =
            Vec::with_capacity(self.redfish_root.len() + extra_segments.len());
        for s in &self.redfish_root {
            segments.push(s.as_str());
        }
        segments.extend_from_slice(extra_segments);

        crate::util::url::join_segments(&self.base_url, &segments)
    }

    fn resolve_uri(&self, uri: &str) -> Result<Url> {
        resolve_uri(&self.base_url, uri)
    }

    fn build_headers(&self, json_body: bool) -> Result<HeaderMap> {
        let mut headers = self.default_headers.clone();

        // Ensure JSON accept by default.
        headers
            .entry(ACCEPT)
            .or_insert(HeaderValue::from_static("application/json"));

        if json_body {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        self.auth.apply_headers(&mut headers)?;
        Ok(headers)
    }
}

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct Client {
    base: ClientBase,
    transport: crate::transport::AsyncTransport,
}

#[cfg(feature = "async")]
impl Client {
    /// Start building a new async client.
    pub fn builder(base_url: &str) -> Result<ClientBuilder> {
        ClientBuilder::new(base_url)
    }

    /// Redfish service root service.
    pub fn service_root(&self) -> ServiceRootService<'_, Client> {
        ServiceRootService::new(self)
    }

    /// Systems service.
    pub fn systems(&self) -> SystemsService<'_, Client> {
        SystemsService::new(self)
    }

    /// A convenience wrapper around a specific system member.
    ///
    /// This complements [`Client::systems`] by making common member sub-resources easier to access.
    pub fn system(&self, system_id: impl Into<String>) -> SystemResourceService<'_, Client> {
        SystemResourceService::new(self, system_id)
    }

    /// Chassis service.
    pub fn chassis(&self) -> ChassisService<'_, Client> {
        ChassisService::new(self)
    }

    /// A convenience wrapper around a specific chassis member.
    pub fn chassis_member(
        &self,
        chassis_id: impl Into<String>,
    ) -> ChassisResourceService<'_, Client> {
        ChassisResourceService::new(self, chassis_id)
    }

    /// Managers service.
    pub fn managers(&self) -> ManagersService<'_, Client> {
        ManagersService::new(self)
    }

    /// A convenience wrapper around a specific manager member.
    pub fn manager(&self, manager_id: impl Into<String>) -> ManagerResourceService<'_, Client> {
        ManagerResourceService::new(self, manager_id)
    }

    /// Sessions service.
    pub fn sessions(&self) -> SessionsService<'_, Client> {
        SessionsService::new(self)
    }

    /// AccountService.
    pub fn account_service(&self) -> AccountServiceService<'_, Client> {
        AccountServiceService::new(self)
    }

    /// EventService.
    pub fn event_service(&self) -> EventServiceService<'_, Client> {
        EventServiceService::new(self)
    }

    /// TaskService.
    pub fn task_service(&self) -> TaskServiceService<'_, Client> {
        TaskServiceService::new(self)
    }

    /// UpdateService.
    pub fn update_service(&self) -> UpdateServiceService<'_, Client> {
        UpdateServiceService::new(self)
    }

    /// Registries discovery service.
    pub fn registries(&self) -> RegistriesService<'_, Client> {
        RegistriesService::new(self)
    }

    /// JSON Schemas discovery service.
    pub fn json_schemas(&self) -> JsonSchemasService<'_, Client> {
        JsonSchemasService::new(self)
    }

    /// Fetch an arbitrary URI (absolute or relative to `base_url`) as JSON.
    pub async fn get_uri<T: DeserializeOwned>(&self, uri: &str) -> Result<T> {
        let url = self.resolve_uri(uri)?;
        self.get_json(url).await
    }

    /// Send an arbitrary request and decode the response as JSON `Value`.
    ///
    /// This is an escape hatch for OEM/unmodeled endpoints.
    ///
    /// - `uri` can be absolute (`https://...`) or relative to the client's `base_url`.
    /// - For success responses with an empty body (e.g. `204 No Content`), this returns `Value::Null`.
    pub async fn request_json_value(
        &self,
        method: Method,
        uri: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let url = self.resolve_uri(uri)?;
        self.request_json_value_url(method, url, body).await
    }

    /// Same as [`Client::request_json_value`], but takes a fully resolved URL.
    pub async fn request_json_value_url(
        &self,
        method: Method,
        url: Url,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let json_body = body.is_some();
        let headers = self.base.build_headers(json_body)?;

        let body_bytes = match body {
            Some(v) => Some(serde_json::to_vec(v).map_err(|e| {
                Error::invalid_config(format!("Failed to serialize request body: {e}"))
            })?),
            None => None,
        };

        let req = Request {
            method: method.clone(),
            url: url.clone(),
            headers,
            body: body_bytes,
        };

        let raw = self.transport.send(req).await?;
        if raw.body.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        decode_json(method, &url, &raw.body, self.base.body_snippet_limit)
    }

    /// Fetch all `Members` across a paginated collection.
    ///
    /// Redfish collections may include `Members@odata.nextLink` which points to the next page.
    pub async fn collect_all_collection_members<T: DeserializeOwned>(
        &self,
        collection_uri: &str,
    ) -> Result<Vec<T>> {
        let mut out: Vec<T> = Vec::new();
        let mut next: Option<String> = Some(collection_uri.to_string());

        while let Some(uri) = next {
            let page: crate::types::Collection<T> = self.get_uri(&uri).await?;
            let next_link = page.next_link().map(|s| s.to_string());
            out.extend(page.members);
            next = next_link;
        }

        Ok(out)
    }

    pub(crate) fn redfish_url(&self, extra_segments: &[&str]) -> Result<Url> {
        self.base.redfish_url(extra_segments)
    }

    pub(crate) fn resolve_uri(&self, uri: &str) -> Result<Url> {
        self.base.resolve_uri(uri)
    }

    pub(crate) fn body_snippet_limit(&self) -> usize {
        self.base.body_snippet_limit
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let headers = self.base.build_headers(false)?;
        let req = Request {
            method: Method::GET,
            url: url.clone(),
            headers,
            body: None,
        };

        let raw = self.transport.send(req).await?;
        decode_json(Method::GET, &url, &raw.body, self.base.body_snippet_limit)
    }

    pub(crate) async fn post_json_raw<B: Serialize>(
        &self,
        url: Url,
        body: &B,
    ) -> Result<RawResponse> {
        let body_bytes = serde_json::to_vec(body)
            .map_err(|e| Error::invalid_config(format!("Failed to serialize request body: {e}")))?;

        let headers = self.base.build_headers(true)?;
        let req = Request {
            method: Method::POST,
            url,
            headers,
            body: Some(body_bytes),
        };

        self.transport.send(req).await
    }

    pub(crate) async fn patch_json_raw<B: Serialize>(
        &self,
        url: Url,
        body: &B,
    ) -> Result<RawResponse> {
        let body_bytes = serde_json::to_vec(body)
            .map_err(|e| Error::invalid_config(format!("Failed to serialize request body: {e}")))?;

        let headers = self.base.build_headers(true)?;
        let req = Request {
            method: Method::PATCH,
            url,
            headers,
            body: Some(body_bytes),
        };

        self.transport.send(req).await
    }

    pub(crate) async fn delete_raw(&self, url: Url) -> Result<RawResponse> {
        let headers = self.base.build_headers(false)?;
        let req = Request {
            method: Method::DELETE,
            url,
            headers,
            body: None,
        };

        self.transport.send(req).await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingClient {
    base: ClientBase,
    transport: crate::transport::BlockingTransport,
}

#[cfg(feature = "blocking")]
impl BlockingClient {
    /// Start building a new blocking client.
    pub fn builder(base_url: &str) -> Result<BlockingClientBuilder> {
        BlockingClientBuilder::new(base_url)
    }

    pub fn service_root(&self) -> ServiceRootService<'_, BlockingClient> {
        ServiceRootService::new(self)
    }

    pub fn systems(&self) -> SystemsService<'_, BlockingClient> {
        SystemsService::new(self)
    }

    /// A convenience wrapper around a specific system member.
    pub fn system(
        &self,
        system_id: impl Into<String>,
    ) -> SystemResourceService<'_, BlockingClient> {
        SystemResourceService::new(self, system_id)
    }

    pub fn chassis(&self) -> ChassisService<'_, BlockingClient> {
        ChassisService::new(self)
    }

    /// A convenience wrapper around a specific chassis member.
    pub fn chassis_member(
        &self,
        chassis_id: impl Into<String>,
    ) -> ChassisResourceService<'_, BlockingClient> {
        ChassisResourceService::new(self, chassis_id)
    }

    pub fn managers(&self) -> ManagersService<'_, BlockingClient> {
        ManagersService::new(self)
    }

    /// A convenience wrapper around a specific manager member.
    pub fn manager(
        &self,
        manager_id: impl Into<String>,
    ) -> ManagerResourceService<'_, BlockingClient> {
        ManagerResourceService::new(self, manager_id)
    }

    pub fn sessions(&self) -> SessionsService<'_, BlockingClient> {
        SessionsService::new(self)
    }

    /// AccountService.
    pub fn account_service(&self) -> AccountServiceService<'_, BlockingClient> {
        AccountServiceService::new(self)
    }

    /// EventService.
    pub fn event_service(&self) -> EventServiceService<'_, BlockingClient> {
        EventServiceService::new(self)
    }

    /// TaskService.
    pub fn task_service(&self) -> TaskServiceService<'_, BlockingClient> {
        TaskServiceService::new(self)
    }

    /// UpdateService.
    pub fn update_service(&self) -> UpdateServiceService<'_, BlockingClient> {
        UpdateServiceService::new(self)
    }

    /// Registries discovery service.
    pub fn registries(&self) -> RegistriesService<'_, BlockingClient> {
        RegistriesService::new(self)
    }

    /// JSON Schemas discovery service.
    pub fn json_schemas(&self) -> JsonSchemasService<'_, BlockingClient> {
        JsonSchemasService::new(self)
    }

    /// Fetch an arbitrary URI (absolute or relative to `base_url`) as JSON.
    pub fn get_uri<T: DeserializeOwned>(&self, uri: &str) -> Result<T> {
        let url = self.resolve_uri(uri)?;
        self.get_json(url)
    }

    /// Send an arbitrary request and decode the response as JSON `Value`.
    ///
    /// This is an escape hatch for OEM/unmodeled endpoints.
    ///
    /// - `uri` can be absolute (`https://...`) or relative to the client's `base_url`.
    /// - For success responses with an empty body (e.g. `204 No Content`), this returns `Value::Null`.
    pub fn request_json_value(
        &self,
        method: Method,
        uri: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let url = self.resolve_uri(uri)?;
        self.request_json_value_url(method, url, body)
    }

    /// Same as [`BlockingClient::request_json_value`], but takes a fully resolved URL.
    pub fn request_json_value_url(
        &self,
        method: Method,
        url: Url,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let json_body = body.is_some();
        let headers = self.base.build_headers(json_body)?;

        let body_bytes = match body {
            Some(v) => Some(serde_json::to_vec(v).map_err(|e| {
                Error::invalid_config(format!("Failed to serialize request body: {e}"))
            })?),
            None => None,
        };

        let req = Request {
            method: method.clone(),
            url: url.clone(),
            headers,
            body: body_bytes,
        };

        let raw = self.transport.send(req)?;
        if raw.body.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        decode_json(method, &url, &raw.body, self.base.body_snippet_limit)
    }

    /// Fetch all `Members` across a paginated collection.
    pub fn collect_all_collection_members<T: DeserializeOwned>(
        &self,
        collection_uri: &str,
    ) -> Result<Vec<T>> {
        let mut out: Vec<T> = Vec::new();
        let mut next: Option<String> = Some(collection_uri.to_string());

        while let Some(uri) = next {
            let page: crate::types::Collection<T> = self.get_uri(&uri)?;
            let next_link = page.next_link().map(|s| s.to_string());
            out.extend(page.members);
            next = next_link;
        }

        Ok(out)
    }

    pub(crate) fn redfish_url(&self, extra_segments: &[&str]) -> Result<Url> {
        self.base.redfish_url(extra_segments)
    }

    pub(crate) fn resolve_uri(&self, uri: &str) -> Result<Url> {
        self.base.resolve_uri(uri)
    }

    pub(crate) fn body_snippet_limit(&self) -> usize {
        self.base.body_snippet_limit
    }

    pub(crate) fn get_json<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let headers = self.base.build_headers(false)?;
        let req = Request {
            method: Method::GET,
            url: url.clone(),
            headers,
            body: None,
        };

        let raw = self.transport.send(req)?;
        decode_json(Method::GET, &url, &raw.body, self.base.body_snippet_limit)
    }

    pub(crate) fn post_json_raw<B: Serialize>(&self, url: Url, body: &B) -> Result<RawResponse> {
        let body_bytes = serde_json::to_vec(body)
            .map_err(|e| Error::invalid_config(format!("Failed to serialize request body: {e}")))?;

        let headers = self.base.build_headers(true)?;
        let req = Request {
            method: Method::POST,
            url,
            headers,
            body: Some(body_bytes),
        };

        self.transport.send(req)
    }

    pub(crate) fn patch_json_raw<B: Serialize>(&self, url: Url, body: &B) -> Result<RawResponse> {
        let body_bytes = serde_json::to_vec(body)
            .map_err(|e| Error::invalid_config(format!("Failed to serialize request body: {e}")))?;

        let headers = self.base.build_headers(true)?;
        let req = Request {
            method: Method::PATCH,
            url,
            headers,
            body: Some(body_bytes),
        };

        self.transport.send(req)
    }

    pub(crate) fn delete_raw(&self, url: Url) -> Result<RawResponse> {
        let headers = self.base.build_headers(false)?;
        let req = Request {
            method: Method::DELETE,
            url,
            headers,
            body: None,
        };

        self.transport.send(req)
    }
}

/// Builder for the async [`Client`].
#[cfg(feature = "async")]
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: Url,
    redfish_root: Vec<String>,
    auth: Auth,
    default_headers: HeaderMap,
    timeout: Duration,
    connect_timeout: Duration,
    retry: RetryPolicy,
    body_snippet_limit: usize,

    #[cfg(feature = "dangerous")]
    danger_accept_invalid_certs: bool,
    #[cfg(feature = "dangerous")]
    danger_accept_invalid_hostnames: bool,

    user_agent: Option<String>,
}

#[cfg(feature = "async")]
impl ClientBuilder {
    fn new(base_url: &str) -> Result<Self> {
        ensure_rustls_provider();
        let base_url = normalize_base_url(base_url)?;
        Ok(Self {
            base_url,
            redfish_root: vec!["redfish".to_string(), "v1".to_string()],
            auth: Auth::default(),
            default_headers: HeaderMap::new(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            retry: RetryPolicy::default(),
            body_snippet_limit: DEFAULT_BODY_SNIPPET_LIMIT,
            user_agent: None,

            #[cfg(feature = "dangerous")]
            danger_accept_invalid_certs: false,
            #[cfg(feature = "dangerous")]
            danger_accept_invalid_hostnames: false,
        })
    }

    /// Set authentication.
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    /// Override the Redfish root path (default: `redfish/v1`).
    ///
    /// Examples:
    /// - `redfish/v1`
    /// - `/redfish/v1/`
    pub fn redfish_root_path(mut self, path: &str) -> Result<Self> {
        let trimmed = path.trim().trim_matches('/');
        if trimmed.is_empty() {
            return Err(Error::invalid_config("redfish_root_path cannot be empty"));
        }
        self.redfish_root = trimmed
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Ok(self)
    }

    /// Add a default header applied to all requests.
    pub fn default_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.default_headers.insert(name, value);
        self
    }

    /// Set request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set connect timeout.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Configure retries.
    pub fn retry_policy(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Configure how much of an error body to keep for diagnostics.
    ///
    /// Set to `0` to disable.
    pub fn body_snippet_limit(mut self, limit: usize) -> Self {
        self.body_snippet_limit = limit;
        self
    }

    /// Override the User-Agent header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Allow invalid TLS certificates (NOT RECOMMENDED).
    #[cfg(feature = "dangerous")]
    pub fn danger_accept_invalid_certs(mut self, yes: bool) -> Self {
        self.danger_accept_invalid_certs = yes;
        self
    }

    /// Allow invalid TLS hostnames (NOT RECOMMENDED).
    #[cfg(feature = "dangerous")]
    pub fn danger_accept_invalid_hostnames(mut self, yes: bool) -> Self {
        self.danger_accept_invalid_hostnames = yes;
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<Client> {
        let mut default_headers = self.default_headers.clone();
        default_headers
            .entry(ACCEPT)
            .or_insert(HeaderValue::from_static("application/json"));

        let mut builder = reqwest::Client::builder()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .default_headers(default_headers);

        if let Some(ua) = self.user_agent {
            builder = builder.user_agent(ua);
        } else {
            builder = builder.user_agent(concat!("redfish/", env!("CARGO_PKG_VERSION")));
        }

        #[cfg(feature = "dangerous")]
        {
            if self.danger_accept_invalid_certs {
                builder = builder.danger_accept_invalid_certs(true);
            }
            if self.danger_accept_invalid_hostnames {
                builder = builder.danger_accept_invalid_hostnames(true);
            }
        }

        let client = builder
            .build()
            .map_err(|e| Error::invalid_config(format!("Failed to build HTTP client: {e}")))?;

        let base = ClientBase {
            base_url: self.base_url,
            redfish_root: self.redfish_root,
            auth: self.auth,
            default_headers: self.default_headers,
            body_snippet_limit: self.body_snippet_limit,
        };

        let transport =
            crate::transport::AsyncTransport::new(client, self.retry, self.body_snippet_limit);

        Ok(Client { base, transport })
    }
}

/// Builder for [`BlockingClient`].
#[cfg(feature = "blocking")]
#[derive(Debug, Clone)]
pub struct BlockingClientBuilder {
    base_url: Url,
    redfish_root: Vec<String>,
    auth: Auth,
    default_headers: HeaderMap,
    timeout: Duration,
    connect_timeout: Duration,
    retry: RetryPolicy,
    body_snippet_limit: usize,

    #[cfg(feature = "dangerous")]
    danger_accept_invalid_certs: bool,
    #[cfg(feature = "dangerous")]
    danger_accept_invalid_hostnames: bool,

    user_agent: Option<String>,
}

#[cfg(feature = "blocking")]
impl BlockingClientBuilder {
    fn new(base_url: &str) -> Result<Self> {
        ensure_rustls_provider();
        let base_url = normalize_base_url(base_url)?;
        Ok(Self {
            base_url,
            redfish_root: vec!["redfish".to_string(), "v1".to_string()],
            auth: Auth::default(),
            default_headers: HeaderMap::new(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            retry: RetryPolicy::default(),
            body_snippet_limit: DEFAULT_BODY_SNIPPET_LIMIT,
            user_agent: None,

            #[cfg(feature = "dangerous")]
            danger_accept_invalid_certs: false,
            #[cfg(feature = "dangerous")]
            danger_accept_invalid_hostnames: false,
        })
    }

    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn redfish_root_path(mut self, path: &str) -> Result<Self> {
        let trimmed = path.trim().trim_matches('/');
        if trimmed.is_empty() {
            return Err(Error::invalid_config("redfish_root_path cannot be empty"));
        }
        self.redfish_root = trimmed
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Ok(self)
    }

    pub fn default_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.default_headers.insert(name, value);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn retry_policy(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    pub fn body_snippet_limit(mut self, limit: usize) -> Self {
        self.body_snippet_limit = limit;
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    #[cfg(feature = "dangerous")]
    pub fn danger_accept_invalid_certs(mut self, yes: bool) -> Self {
        self.danger_accept_invalid_certs = yes;
        self
    }

    #[cfg(feature = "dangerous")]
    pub fn danger_accept_invalid_hostnames(mut self, yes: bool) -> Self {
        self.danger_accept_invalid_hostnames = yes;
        self
    }

    pub fn build(self) -> Result<BlockingClient> {
        let mut default_headers = self.default_headers.clone();
        default_headers
            .entry(ACCEPT)
            .or_insert(HeaderValue::from_static("application/json"));

        let mut builder = reqwest::blocking::Client::builder()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .default_headers(default_headers);

        if let Some(ua) = self.user_agent {
            builder = builder.user_agent(ua);
        } else {
            builder = builder.user_agent(concat!("redfish/", env!("CARGO_PKG_VERSION")));
        }

        #[cfg(feature = "dangerous")]
        {
            if self.danger_accept_invalid_certs {
                builder = builder.danger_accept_invalid_certs(true);
            }
            if self.danger_accept_invalid_hostnames {
                builder = builder.danger_accept_invalid_hostnames(true);
            }
        }

        let client = builder
            .build()
            .map_err(|e| Error::invalid_config(format!("Failed to build HTTP client: {e}")))?;

        let base = ClientBase {
            base_url: self.base_url,
            redfish_root: self.redfish_root,
            auth: self.auth,
            default_headers: self.default_headers,
            body_snippet_limit: self.body_snippet_limit,
        };

        let transport =
            crate::transport::BlockingTransport::new(client, self.retry, self.body_snippet_limit);

        Ok(BlockingClient { base, transport })
    }
}

fn decode_json<T: DeserializeOwned>(
    method: Method,
    url: &Url,
    body: &[u8],
    body_snippet_limit: usize,
) -> Result<T> {
    let ctx = RequestContext::new(method.clone(), url.to_string());
    let mut de = serde_json::Deserializer::from_slice(body);
    match serde_path_to_error::deserialize::<_, T>(&mut de) {
        Ok(v) => Ok(v),
        Err(e) => {
            let snippet = crate::util::redact::redacted_body_snippet(body, body_snippet_limit);
            let msg = format!("Failed to decode JSON response: {e}");
            Err(Error::decode(ctx, msg, snippet, e))
        }
    }
}
