use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{Collection, ComputerSystem, OdataId, ResetType, actions::ResetRequest};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access `Systems` collection and member resources.
#[derive(Debug, Clone, Copy)]
pub struct SystemsService<'a, C> {
    client: &'a C,
}

impl<'a, C> SystemsService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "async")]
impl<'a> SystemsService<'a, Client> {
    /// `GET /redfish/v1/Systems`
    pub async fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Systems"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}`
    pub async fn get(&self, system_id: &str) -> Result<ComputerSystem> {
        let url = self.client.redfish_url(&["Systems", system_id])?;
        self.client.get_json(url).await
    }

    /// Reset a computer system.
    ///
    /// `POST /redfish/v1/Systems/{id}/Actions/ComputerSystem.Reset`
    pub async fn reset(
        &self,
        system_id: &str,
        reset_type: ResetType,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url =
            self.client
                .redfish_url(&["Systems", system_id, "Actions", "ComputerSystem.Reset"])?;

        let req = ResetRequest { reset_type };
        let raw = self.client.post_json_raw(url.clone(), &req).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}

#[cfg(feature = "blocking")]
impl<'a> SystemsService<'a, BlockingClient> {
    /// `GET /redfish/v1/Systems`
    pub fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Systems"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/Systems/{id}`
    pub fn get(&self, system_id: &str) -> Result<ComputerSystem> {
        let url = self.client.redfish_url(&["Systems", system_id])?;
        self.client.get_json(url)
    }

    /// Reset a computer system.
    ///
    /// `POST /redfish/v1/Systems/{id}/Actions/ComputerSystem.Reset`
    pub fn reset(
        &self,
        system_id: &str,
        reset_type: ResetType,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url =
            self.client
                .redfish_url(&["Systems", system_id, "Actions", "ComputerSystem.Reset"])?;

        let req = ResetRequest { reset_type };
        let raw = self.client.post_json_raw(url.clone(), &req)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}
