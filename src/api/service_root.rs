use crate::Result;
use crate::types::ServiceRoot;

#[cfg(feature = "_blocking")]
use crate::BlockingClient;
#[cfg(feature = "_async")]
use crate::Client;

/// Access the Redfish service root (`/redfish/v1` by default).
#[derive(Debug, Clone, Copy)]
pub struct ServiceRootService<'a, C> {
    client: &'a C,
}

impl<'a, C> ServiceRootService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "_async")]
impl<'a> ServiceRootService<'a, Client> {
    /// `GET /redfish/v1`
    pub async fn get(&self) -> Result<ServiceRoot> {
        let url = self.client.redfish_url(&[])?;
        self.client.get_json(url).await
    }
}

#[cfg(feature = "_blocking")]
impl<'a> ServiceRootService<'a, BlockingClient> {
    /// `GET /redfish/v1`
    pub fn get(&self) -> Result<ServiceRoot> {
        let url = self.client.redfish_url(&[])?;
        self.client.get_json(url)
    }
}
