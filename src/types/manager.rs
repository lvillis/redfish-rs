use serde::Deserialize;
use serde_json::{Map, Value};

use super::ResourceStatus;

/// Manager (`Manager` resource).
#[derive(Debug, Clone, Deserialize)]
pub struct Manager {
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

    #[serde(rename = "ManagerType", default)]
    pub manager_type: Option<String>,

    #[serde(rename = "FirmwareVersion", default)]
    pub firmware_version: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
