use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Minimal OData identifier used across Redfish resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OdataId {
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
}

impl OdataId {
    pub fn as_str(&self) -> &str {
        &self.odata_id
    }
}

/// Common Redfish resource envelope.
///
/// Many Redfish resources include these common fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "@odata.id")]
    pub odata_id: String,

    #[serde(rename = "@odata.type", default)]
    pub odata_type: Option<String>,

    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Description", default)]
    pub description: Option<String>,

    /// Any additional fields not explicitly modeled.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Standard status block used by many Redfish resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStatus {
    #[serde(rename = "Health", default)]
    pub health: Option<String>,

    #[serde(rename = "HealthRollup", default)]
    pub health_rollup: Option<String>,

    #[serde(rename = "State", default)]
    pub state: Option<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
