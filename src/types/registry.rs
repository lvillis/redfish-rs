use serde::Deserialize;
use serde_json::{Map, Value};

/// Message registry file resource.
///
/// Typically a member of `/redfish/v1/Registries`.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageRegistryFile {
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

    #[serde(rename = "Registry", default)]
    pub registry: Option<String>,

    #[serde(rename = "Location", default)]
    pub location: Vec<MessageRegistryLocation>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageRegistryLocation {
    #[serde(rename = "Language", default)]
    pub language: Option<String>,

    #[serde(rename = "Uri", default)]
    pub uri: Option<String>,

    #[serde(rename = "PublicationUri", default)]
    pub publication_uri: Option<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
