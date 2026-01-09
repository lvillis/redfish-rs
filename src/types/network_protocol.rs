use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{Resource, ResourceStatus};

/// Manager network protocol configuration.
///
/// Typical path: `GET /redfish/v1/Managers/{id}/NetworkProtocol`
#[derive(Debug, Clone, Deserialize)]
pub struct ManagerNetworkProtocol {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(rename = "HTTP", default)]
    pub http: Option<ProtocolSettings>,

    #[serde(rename = "HTTPS", default)]
    pub https: Option<ProtocolSettings>,

    #[serde(rename = "SSH", default)]
    pub ssh: Option<ProtocolSettings>,

    #[serde(rename = "IPMI", default)]
    pub ipmi: Option<ProtocolSettings>,

    #[serde(rename = "SNMP", default)]
    pub snmp: Option<ProtocolSettings>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProtocolSettings {
    #[serde(rename = "ProtocolEnabled", default)]
    pub protocol_enabled: Option<bool>,

    #[serde(rename = "Port", default)]
    pub port: Option<u16>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Patch payload for manager network protocol.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ManagerNetworkProtocolUpdateRequest {
    #[serde(rename = "HTTP", skip_serializing_if = "Option::is_none")]
    pub http: Option<ProtocolSettingsUpdate>,

    #[serde(rename = "HTTPS", skip_serializing_if = "Option::is_none")]
    pub https: Option<ProtocolSettingsUpdate>,

    #[serde(rename = "SSH", skip_serializing_if = "Option::is_none")]
    pub ssh: Option<ProtocolSettingsUpdate>,

    #[serde(rename = "IPMI", skip_serializing_if = "Option::is_none")]
    pub ipmi: Option<ProtocolSettingsUpdate>,

    #[serde(rename = "SNMP", skip_serializing_if = "Option::is_none")]
    pub snmp: Option<ProtocolSettingsUpdate>,

    /// Optional vendor extensions.
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

impl ManagerNetworkProtocolUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ProtocolSettingsUpdate {
    #[serde(rename = "ProtocolEnabled", skip_serializing_if = "Option::is_none")]
    pub protocol_enabled: Option<bool>,

    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// Optional vendor extensions.
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

impl ProtocolSettingsUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
