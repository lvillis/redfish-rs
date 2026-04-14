use crate::Result;
use crate::types::{Collection, JsonSchemaFile, OdataId};

#[cfg(feature = "_blocking")]
use crate::BlockingClient;
#[cfg(feature = "_async")]
use crate::Client;

/// Access `JsonSchemas`.
#[derive(Debug, Clone, Copy)]
pub struct JsonSchemasService<'a, C> {
    client: &'a C,
}

impl<'a, C> JsonSchemasService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "_async")]
impl<'a> JsonSchemasService<'a, Client> {
    /// `GET /redfish/v1/JsonSchemas`
    pub async fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["JsonSchemas"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/JsonSchemas/{id}`
    pub async fn get(&self, schema_id: &str) -> Result<JsonSchemaFile> {
        let url = self.client.redfish_url(&["JsonSchemas", schema_id])?;
        self.client.get_json(url).await
    }
}

#[cfg(feature = "_blocking")]
impl<'a> JsonSchemasService<'a, BlockingClient> {
    /// `GET /redfish/v1/JsonSchemas`
    pub fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["JsonSchemas"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/JsonSchemas/{id}`
    pub fn get(&self, schema_id: &str) -> Result<JsonSchemaFile> {
        let url = self.client.redfish_url(&["JsonSchemas", schema_id])?;
        self.client.get_json(url)
    }
}
