use serde::Deserialize;
use serde_json::{Map, Value};

use super::ResourceStatus;

/// Computer system (`ComputerSystem` resource).
#[derive(Debug, Clone, Deserialize)]
pub struct ComputerSystem {
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

    #[serde(rename = "SystemType", default)]
    pub system_type: Option<String>,

    #[serde(rename = "Manufacturer", default)]
    pub manufacturer: Option<String>,

    #[serde(rename = "Model", default)]
    pub model: Option<String>,

    #[serde(rename = "SerialNumber", default)]
    pub serial_number: Option<String>,

    #[serde(rename = "UUID", default)]
    pub uuid: Option<String>,

    #[serde(rename = "BiosVersion", default)]
    pub bios_version: Option<String>,

    #[serde(rename = "PowerState", default)]
    pub power_state: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(rename = "ProcessorSummary", default)]
    pub processor_summary: Option<ProcessorSummary>,

    #[serde(rename = "MemorySummary", default)]
    pub memory_summary: Option<MemorySummary>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessorSummary {
    #[serde(rename = "Count", default)]
    pub count: Option<u32>,

    #[serde(rename = "Model", default)]
    pub model: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemorySummary {
    #[serde(rename = "TotalSystemMemoryGiB", default)]
    pub total_system_memory_gib: Option<f64>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
