use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{OdataId, ResourceStatus};

/// EventService resource.
///
/// Typically available at `/redfish/v1/EventService`.
#[derive(Debug, Clone, Deserialize)]
pub struct EventService {
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

    #[serde(rename = "ServiceEnabled", default)]
    pub service_enabled: Option<bool>,

    #[serde(rename = "DeliveryRetryAttempts", default)]
    pub delivery_retry_attempts: Option<u32>,

    #[serde(rename = "DeliveryRetryIntervalSeconds", default)]
    pub delivery_retry_interval_seconds: Option<u32>,

    #[serde(rename = "EventTypesForSubscription", default)]
    pub event_types_for_subscription: Vec<String>,

    #[serde(rename = "RegistryPrefixes", default)]
    pub registry_prefixes: Vec<String>,

    #[serde(rename = "ResourceTypes", default)]
    pub resource_types: Vec<String>,

    #[serde(rename = "Subscriptions", default)]
    pub subscriptions: Option<OdataId>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Request body for `POST /EventService/Actions/EventService.SubmitTestEvent`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SubmitTestEventRequest {
    /// A unique identifier for the event.
    #[serde(rename = "EventId", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// Event timestamp (implementation-defined string, often ISO-8601).
    #[serde(rename = "EventTimestamp", skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<String>,

    /// Event type.
    #[serde(rename = "EventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Human-readable message.
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Message registry identifier.
    #[serde(rename = "MessageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Origin of condition (often an `@odata.id` URI).
    #[serde(rename = "OriginOfCondition", skip_serializing_if = "Option::is_none")]
    pub origin_of_condition: Option<Value>,

    /// Severity (implementation-defined string).
    #[serde(rename = "Severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}
