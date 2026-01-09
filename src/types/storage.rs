use serde::Deserialize;
use serde_json::{Map, Value};

use super::{OdataId, Resource, ResourceStatus};

/// Storage resource.
///
/// Typical path: `GET /redfish/v1/Systems/{id}/Storage/{storage_id}`
#[derive(Debug, Clone, Deserialize)]
pub struct Storage {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    /// Links to drives (often `@odata.id` references).
    #[serde(rename = "Drives", default)]
    pub drives: Vec<OdataId>,

    /// Link to Volumes collection.
    #[serde(rename = "Volumes", default)]
    pub volumes: Option<OdataId>,

    #[serde(rename = "StorageControllers", default)]
    pub storage_controllers: Vec<StorageController>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageController {
    #[serde(rename = "MemberId", default)]
    pub member_id: Option<String>,

    #[serde(rename = "Manufacturer", default)]
    pub manufacturer: Option<String>,

    #[serde(rename = "Model", default)]
    pub model: Option<String>,

    #[serde(rename = "FirmwareVersion", default)]
    pub firmware_version: Option<String>,

    #[serde(rename = "SerialNumber", default)]
    pub serial_number: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Drive resource.
///
/// Common locations:
/// - `GET /redfish/v1/Systems/{sys_id}/Storage/{storage_id}/Drives/{drive_id}`
/// - `GET /redfish/v1/Chassis/{chassis_id}/Drives/{drive_id}`
#[derive(Debug, Clone, Deserialize)]
pub struct Drive {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Model", default)]
    pub model: Option<String>,

    #[serde(rename = "Manufacturer", default)]
    pub manufacturer: Option<String>,

    #[serde(rename = "SerialNumber", default)]
    pub serial_number: Option<String>,

    #[serde(rename = "PartNumber", default)]
    pub part_number: Option<String>,

    #[serde(rename = "CapacityBytes", default)]
    pub capacity_bytes: Option<u64>,

    #[serde(rename = "Protocol", default)]
    pub protocol: Option<String>,

    #[serde(rename = "MediaType", default)]
    pub media_type: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,
}
