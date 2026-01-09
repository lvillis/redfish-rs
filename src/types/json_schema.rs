use serde::Deserialize;
use serde_json::{Map, Value};

/// JSON schema file resource.
///
/// Typically a member of `/redfish/v1/JsonSchemas`.
#[derive(Debug, Clone, Deserialize)]
pub struct JsonSchemaFile {
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

    #[serde(rename = "Schema", default)]
    pub schema: Option<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
