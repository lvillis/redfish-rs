use crate::Result;
use crate::types::{Collection, MessageRegistryFile, OdataId};

#[cfg(feature = "_blocking")]
use crate::BlockingClient;
#[cfg(feature = "_async")]
use crate::Client;

/// Access `Registries`.
#[derive(Debug, Clone, Copy)]
pub struct RegistriesService<'a, C> {
    client: &'a C,
}

impl<'a, C> RegistriesService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "_async")]
impl<'a> RegistriesService<'a, Client> {
    /// `GET /redfish/v1/Registries`
    pub async fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Registries"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Registries/{id}`
    pub async fn get(&self, registry_id: &str) -> Result<MessageRegistryFile> {
        let url = self.client.redfish_url(&["Registries", registry_id])?;
        self.client.get_json(url).await
    }
}

#[cfg(feature = "_blocking")]
impl<'a> RegistriesService<'a, BlockingClient> {
    /// `GET /redfish/v1/Registries`
    pub fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Registries"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/Registries/{id}`
    pub fn get(&self, registry_id: &str) -> Result<MessageRegistryFile> {
        let url = self.client.redfish_url(&["Registries", registry_id])?;
        self.client.get_json(url)
    }
}
