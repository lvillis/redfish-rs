use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{Resource, ResourceStatus};

/// ComputerSystem BIOS resource.
///
/// Typical path: `GET /redfish/v1/Systems/{id}/Bios`
#[derive(Debug, Clone, Deserialize)]
pub struct Bios {
    #[serde(flatten)]
    pub resource: Resource,

    /// BIOS settings/attributes exposed by the implementation.
    #[serde(rename = "Attributes", default)]
    pub attributes: Map<String, Value>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,
}

/// BIOS settings resource (pending/desired settings).
///
/// Typical path: `GET /redfish/v1/Systems/{id}/Bios/Settings`
#[derive(Debug, Clone, Deserialize)]
pub struct BiosSettings {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Attributes", default)]
    pub attributes: Map<String, Value>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,
}

/// Patch payload for BIOS settings.
///
/// Typical operation: `PATCH /redfish/v1/Systems/{id}/Bios/Settings`
#[derive(Debug, Clone, Serialize, Default)]
pub struct BiosSettingsUpdateRequest {
    #[serde(rename = "Attributes", default)]
    pub attributes: Map<String, Value>,

    /// Optional vendor extensions.
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

impl BiosSettingsUpdateRequest {
    /// Create an empty update request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add/override a BIOS attribute.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Add an OEM/vendor-specific key/value to the top-level payload.
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
