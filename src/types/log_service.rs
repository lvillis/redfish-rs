use serde::Deserialize;
use serde_json::{Map, Value};

use super::{OdataId, Resource, ResourceStatus};

/// LogService resource.
///
/// Typical paths:
/// - `GET /redfish/v1/Systems/{id}/LogServices/{log_id}`
/// - `GET /redfish/v1/Managers/{id}/LogServices/{log_id}`
#[derive(Debug, Clone, Deserialize)]
pub struct LogService {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "OverWritePolicy", default)]
    pub overwrite_policy: Option<String>,

    #[serde(rename = "MaxNumberOfRecords", default)]
    pub max_number_of_records: Option<u64>,

    #[serde(rename = "Entries", default)]
    pub entries: Option<OdataId>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,
}

/// LogEntry resource.
#[derive(Debug, Clone, Deserialize)]
pub struct LogEntry {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Created", default)]
    pub created: Option<String>,

    #[serde(rename = "EntryType", default)]
    pub entry_type: Option<String>,

    #[serde(rename = "Severity", default)]
    pub severity: Option<String>,

    #[serde(rename = "Message", default)]
    pub message: Option<String>,

    #[serde(rename = "MessageId", default)]
    pub message_id: Option<String>,

    #[serde(rename = "SensorType", default)]
    pub sensor_type: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
