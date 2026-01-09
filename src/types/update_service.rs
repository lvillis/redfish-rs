use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{OdataId, ResourceStatus};

/// UpdateService resource.
///
/// Typically available at `/redfish/v1/UpdateService`.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateService {
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

    #[serde(rename = "FirmwareInventory", default)]
    pub firmware_inventory: Option<OdataId>,

    #[serde(rename = "SoftwareInventory", default)]
    pub software_inventory: Option<OdataId>,

    #[serde(rename = "HttpPushUri", default)]
    pub http_push_uri: Option<String>,

    #[serde(rename = "MultipartHttpPushUri", default)]
    pub multipart_http_push_uri: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Firmware/software inventory item.
///
/// Redfish uses the `SoftwareInventory` schema for many inventory resources.
#[derive(Debug, Clone, Deserialize)]
pub struct SoftwareInventory {
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

    #[serde(rename = "Version", default)]
    pub version: Option<String>,

    #[serde(rename = "Updateable", default)]
    pub updateable: Option<bool>,

    #[serde(rename = "SoftwareId", default)]
    pub software_id: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Request body for `POST /UpdateService/Actions/UpdateService.SimpleUpdate`.
#[derive(Clone, Serialize, Default)]
pub struct SimpleUpdateRequest {
    /// URI to the firmware/software image.
    #[serde(rename = "ImageURI")]
    image_uri: String,

    #[serde(rename = "TransferProtocol", skip_serializing_if = "Option::is_none")]
    transfer_protocol: Option<String>,

    /// Optional list of target URIs.
    #[serde(rename = "Targets", skip_serializing_if = "Vec::is_empty", default)]
    targets: Vec<String>,

    /// Optional username for the image URI.
    #[serde(rename = "Username", skip_serializing_if = "Option::is_none")]
    username: Option<String>,

    /// Optional password for the image URI.
    #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
    password: Option<String>,

    /// Allow update even if host is powered on.
    #[serde(rename = "ForceUpdate", skip_serializing_if = "Option::is_none")]
    force_update: Option<bool>,
}

impl SimpleUpdateRequest {
    /// Create a simple-update request.
    pub fn new(image_uri: impl Into<String>) -> Self {
        Self {
            image_uri: image_uri.into(),
            transfer_protocol: None,
            targets: Vec::new(),
            username: None,
            password: None,
            force_update: None,
        }
    }

    pub fn image_uri(&self) -> &str {
        &self.image_uri
    }

    pub fn transfer_protocol(mut self, transfer_protocol: impl Into<String>) -> Self {
        self.transfer_protocol = Some(transfer_protocol.into());
        self
    }

    pub fn targets<I, S>(mut self, targets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.targets = targets.into_iter().map(Into::into).collect();
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn force_update(mut self, yes: bool) -> Self {
        self.force_update = Some(yes);
        self
    }
}

impl fmt::Debug for SimpleUpdateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleUpdateRequest")
            .field("image_uri", &self.image_uri)
            .field("transfer_protocol", &self.transfer_protocol)
            .field("targets", &self.targets)
            .field("username", &self.username)
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .field("force_update", &self.force_update)
            .finish()
    }
}
