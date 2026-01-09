use serde::Deserialize;
use serde_json::{Map, Value};

use super::{Resource, ResourceStatus};

/// Chassis Thermal resource.
///
/// Typical path: `GET /redfish/v1/Chassis/{id}/Thermal`
#[derive(Debug, Clone, Deserialize)]
pub struct Thermal {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "Fans", default)]
    pub fans: Vec<Fan>,

    #[serde(rename = "Temperatures", default)]
    pub temperatures: Vec<Temperature>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fan {
    #[serde(rename = "MemberId", default)]
    pub member_id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Reading", default)]
    pub reading: Option<f64>,

    #[serde(rename = "ReadingUnits", default)]
    pub reading_units: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Temperature {
    #[serde(rename = "MemberId", default)]
    pub member_id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "ReadingCelsius", default)]
    pub reading_celsius: Option<f64>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
