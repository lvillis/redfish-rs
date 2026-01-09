use serde::Deserialize;
use serde_json::{Map, Value};

use super::ResourceStatus;

/// Redfish Session resource (returned by SessionService).
#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    #[serde(rename = "@odata.id", default)]
    pub odata_id: Option<String>,

    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "UserName", default)]
    pub user_name: Option<String>,

    #[serde(rename = "Description", default)]
    pub description: Option<String>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
