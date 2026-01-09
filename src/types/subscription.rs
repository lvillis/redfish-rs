use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::ResourceStatus;

/// Event subscription (`EventDestination` resource).
#[derive(Debug, Clone, Deserialize)]
pub struct EventSubscription {
    #[serde(rename = "@odata.id", default)]
    pub odata_id: Option<String>,

    #[serde(rename = "@odata.type", default)]
    pub odata_type: Option<String>,

    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Description", default)]
    pub description: Option<String>,

    #[serde(rename = "Destination", default)]
    pub destination: Option<String>,

    #[serde(rename = "Context", default)]
    pub context: Option<String>,

    #[serde(rename = "Protocol", default)]
    pub protocol: Option<String>,

    #[serde(rename = "SubscriptionType", default)]
    pub subscription_type: Option<String>,

    #[serde(rename = "EventTypes", default)]
    pub event_types: Vec<String>,

    #[serde(rename = "RegistryPrefixes", default)]
    pub registry_prefixes: Vec<String>,

    #[serde(rename = "ResourceTypes", default)]
    pub resource_types: Vec<String>,

    #[serde(rename = "MessageIds", default)]
    pub message_ids: Vec<String>,

    #[serde(rename = "HttpHeaders", default)]
    pub http_headers: Option<BTreeMap<String, String>>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Request body for `POST /EventService/Subscriptions`.
#[derive(Clone, Serialize)]
pub struct EventSubscriptionCreateRequest {
    #[serde(rename = "Destination")]
    destination: String,

    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    context: Option<String>,

    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    protocol: Option<String>,

    #[serde(rename = "SubscriptionType", skip_serializing_if = "Option::is_none")]
    subscription_type: Option<String>,

    #[serde(rename = "EventTypes", skip_serializing_if = "Vec::is_empty", default)]
    event_types: Vec<String>,

    #[serde(
        rename = "RegistryPrefixes",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    registry_prefixes: Vec<String>,

    #[serde(
        rename = "ResourceTypes",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    resource_types: Vec<String>,

    #[serde(rename = "MessageIds", skip_serializing_if = "Vec::is_empty", default)]
    message_ids: Vec<String>,

    /// Additional headers to include in event POSTs.
    ///
    /// Warning: values may include secrets (e.g. Authorization). The `Debug` impl redacts them.
    #[serde(rename = "HttpHeaders", skip_serializing_if = "Option::is_none")]
    http_headers: Option<BTreeMap<String, String>>,
}

impl EventSubscriptionCreateRequest {
    /// Create a subscription create request.
    pub fn new(destination: impl Into<String>) -> Self {
        Self {
            destination: destination.into(),
            context: None,
            protocol: None,
            subscription_type: None,
            event_types: Vec::new(),
            registry_prefixes: Vec::new(),
            resource_types: Vec::new(),
            message_ids: Vec::new(),
            http_headers: None,
        }
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }

    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    pub fn subscription_type(mut self, subscription_type: impl Into<String>) -> Self {
        self.subscription_type = Some(subscription_type.into());
        self
    }

    pub fn event_types<I, S>(mut self, event_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.event_types = event_types.into_iter().map(Into::into).collect();
        self
    }

    pub fn registry_prefixes<I, S>(mut self, registry_prefixes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.registry_prefixes = registry_prefixes.into_iter().map(Into::into).collect();
        self
    }

    pub fn resource_types<I, S>(mut self, resource_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.resource_types = resource_types.into_iter().map(Into::into).collect();
        self
    }

    pub fn message_ids<I, S>(mut self, message_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.message_ids = message_ids.into_iter().map(Into::into).collect();
        self
    }

    pub fn http_headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.http_headers = Some(headers);
        self
    }
}

impl fmt::Debug for EventSubscriptionCreateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventSubscriptionCreateRequest")
            .field("destination", &self.destination)
            .field("context", &self.context)
            .field("protocol", &self.protocol)
            .field("subscription_type", &self.subscription_type)
            .field("event_types", &self.event_types)
            .field("registry_prefixes", &self.registry_prefixes)
            .field("resource_types", &self.resource_types)
            .field("message_ids", &self.message_ids)
            .field("http_headers", &redacted_header_map(&self.http_headers))
            .finish()
    }
}

/// Request body for `PATCH /EventService/Subscriptions/{id}`.
#[derive(Clone, Serialize, Default)]
pub struct EventSubscriptionUpdateRequest {
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    context: Option<String>,

    #[serde(rename = "EventTypes", skip_serializing_if = "Option::is_none")]
    event_types: Option<Vec<String>>,

    #[serde(rename = "RegistryPrefixes", skip_serializing_if = "Option::is_none")]
    registry_prefixes: Option<Vec<String>>,

    #[serde(rename = "ResourceTypes", skip_serializing_if = "Option::is_none")]
    resource_types: Option<Vec<String>>,

    #[serde(rename = "MessageIds", skip_serializing_if = "Option::is_none")]
    message_ids: Option<Vec<String>>,

    #[serde(rename = "HttpHeaders", skip_serializing_if = "Option::is_none")]
    http_headers: Option<BTreeMap<String, String>>,
}

impl EventSubscriptionUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn event_types<I, S>(mut self, event_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.event_types = Some(event_types.into_iter().map(Into::into).collect());
        self
    }

    pub fn registry_prefixes<I, S>(mut self, registry_prefixes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.registry_prefixes = Some(registry_prefixes.into_iter().map(Into::into).collect());
        self
    }

    pub fn resource_types<I, S>(mut self, resource_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.resource_types = Some(resource_types.into_iter().map(Into::into).collect());
        self
    }

    pub fn message_ids<I, S>(mut self, message_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.message_ids = Some(message_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn http_headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.http_headers = Some(headers);
        self
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.context.is_none()
            && self.event_types.is_none()
            && self.registry_prefixes.is_none()
            && self.resource_types.is_none()
            && self.message_ids.is_none()
            && self.http_headers.is_none()
    }
}

impl fmt::Debug for EventSubscriptionUpdateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventSubscriptionUpdateRequest")
            .field("context", &self.context)
            .field("event_types", &self.event_types)
            .field("registry_prefixes", &self.registry_prefixes)
            .field("resource_types", &self.resource_types)
            .field("message_ids", &self.message_ids)
            .field("http_headers", &redacted_header_map(&self.http_headers))
            .finish()
    }
}

fn redacted_header_map(
    headers: &Option<BTreeMap<String, String>>,
) -> Option<BTreeMap<String, String>> {
    let headers = headers.as_ref()?;

    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in headers {
        if is_sensitive_header(k) {
            out.insert(k.clone(), "<redacted>".to_string());
        } else {
            out.insert(k.clone(), v.clone());
        }
    }
    Some(out)
}

fn is_sensitive_header(name: &str) -> bool {
    let n = name.trim().to_ascii_lowercase();
    matches!(
        n.as_str(),
        "authorization" | "x-auth-token" | "cookie" | "set-cookie"
    )
}
