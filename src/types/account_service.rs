use serde::Deserialize;
use serde_json::{Map, Value};

use super::{OdataId, ResourceStatus};

/// AccountService resource.
///
/// Typically available at `/redfish/v1/AccountService`.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountService {
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

    #[serde(rename = "MinPasswordLength", default)]
    pub min_password_length: Option<u32>,

    #[serde(rename = "MaxPasswordLength", default)]
    pub max_password_length: Option<u32>,

    #[serde(rename = "Accounts", default)]
    pub accounts: Option<OdataId>,

    #[serde(rename = "Roles", default)]
    pub roles: Option<OdataId>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
