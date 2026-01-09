use serde::Deserialize;
use serde_json::{Map, Value};

use super::{Resource, ResourceStatus};

/// Chassis Power resource.
///
/// Typical path: `GET /redfish/v1/Chassis/{id}/Power`
#[derive(Debug, Clone, Deserialize)]
pub struct Power {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "PowerControl", default)]
    pub power_control: Vec<PowerControl>,

    #[serde(rename = "PowerSupplies", default)]
    pub power_supplies: Vec<PowerSupply>,

    #[serde(rename = "Voltages", default)]
    pub voltages: Vec<Voltage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PowerControl {
    #[serde(rename = "MemberId", default)]
    pub member_id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "PowerConsumedWatts", default)]
    pub power_consumed_watts: Option<f64>,

    #[serde(rename = "PowerCapacityWatts", default)]
    pub power_capacity_watts: Option<f64>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PowerSupply {
    #[serde(rename = "MemberId", default)]
    pub member_id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Manufacturer", default)]
    pub manufacturer: Option<String>,

    #[serde(rename = "Model", default)]
    pub model: Option<String>,

    #[serde(rename = "SerialNumber", default)]
    pub serial_number: Option<String>,

    #[serde(rename = "PowerCapacityWatts", default)]
    pub power_capacity_watts: Option<f64>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Voltage {
    #[serde(rename = "MemberId", default)]
    pub member_id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "ReadingVolts", default)]
    pub reading_volts: Option<f64>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
