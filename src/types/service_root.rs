use serde::Deserialize;
use serde_json::{Map, Value};

use super::OdataId;

/// `GET /redfish/v1` response.
#[derive(Debug, Clone, Deserialize)]
pub struct ServiceRoot {
    #[serde(rename = "@odata.id", default)]
    pub odata_id: Option<String>,

    #[serde(rename = "RedfishVersion")]
    pub redfish_version: String,

    #[serde(rename = "UUID", default)]
    pub uuid: Option<String>,

    #[serde(rename = "Systems", default)]
    pub systems: Option<OdataId>,

    #[serde(rename = "Chassis", default)]
    pub chassis: Option<OdataId>,

    #[serde(rename = "Managers", default)]
    pub managers: Option<OdataId>,

    #[serde(rename = "SessionService", default)]
    pub session_service: Option<OdataId>,

    #[serde(rename = "AccountService", default)]
    pub account_service: Option<OdataId>,

    #[serde(rename = "EventService", default)]
    pub event_service: Option<OdataId>,

    #[serde(rename = "TaskService", default)]
    pub task_service: Option<OdataId>,

    #[serde(rename = "UpdateService", default)]
    pub update_service: Option<OdataId>,

    #[serde(rename = "Registries", default)]
    pub registries: Option<OdataId>,

    #[serde(rename = "JsonSchemas", default)]
    pub json_schemas: Option<OdataId>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
