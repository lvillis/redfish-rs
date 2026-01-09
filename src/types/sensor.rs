use serde::Deserialize;
use serde_json::{Map, Value};

use super::{Resource, ResourceStatus};

/// Sensor resource.
///
/// Typical locations:
/// - `GET /redfish/v1/Chassis/{id}/Sensors/{sensor_id}`
/// - `GET /redfish/v1/Chassis/{id}/Sensors` (collection)
#[derive(Debug, Clone, Deserialize)]
pub struct Sensor {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Reading", default)]
    pub reading: Option<f64>,

    #[serde(rename = "ReadingType", default)]
    pub reading_type: Option<String>,

    #[serde(rename = "ReadingUnits", default)]
    pub reading_units: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
