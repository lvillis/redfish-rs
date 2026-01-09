use serde::Deserialize;
use serde_json::{Map, Value};

/// Role resource.
///
/// Typically a member of `/redfish/v1/AccountService/Roles`.
#[derive(Debug, Clone, Deserialize)]
pub struct Role {
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

    #[serde(rename = "IsPredefined", default)]
    pub is_predefined: Option<bool>,

    #[serde(rename = "AssignedPrivileges", default)]
    pub assigned_privileges: Vec<String>,

    #[serde(rename = "OemPrivileges", default)]
    pub oem_privileges: Vec<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
