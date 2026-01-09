use serde::Deserialize;
use serde_json::{Map, Value};

use super::{OdataId, ResourceStatus};

/// TaskService resource.
///
/// Typically available at `/redfish/v1/TaskService`.
#[derive(Debug, Clone, Deserialize)]
pub struct TaskService {
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

    #[serde(rename = "ServiceEnabled", default)]
    pub service_enabled: Option<bool>,

    #[serde(rename = "Tasks", default)]
    pub tasks: Option<OdataId>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
