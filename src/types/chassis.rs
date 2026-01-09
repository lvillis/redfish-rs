use serde::Deserialize;
use serde_json::{Map, Value};

use super::ResourceStatus;

/// Chassis (`Chassis` resource).
#[derive(Debug, Clone, Deserialize)]
pub struct Chassis {
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

    #[serde(rename = "ChassisType", default)]
    pub chassis_type: Option<String>,

    #[serde(rename = "Manufacturer", default)]
    pub manufacturer: Option<String>,

    #[serde(rename = "Model", default)]
    pub model: Option<String>,

    #[serde(rename = "SerialNumber", default)]
    pub serial_number: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
