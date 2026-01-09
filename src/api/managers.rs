use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{Collection, Manager, OdataId, ResetType, actions::ResetRequest};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access `Managers` collection and member resources.
#[derive(Debug, Clone, Copy)]
pub struct ManagersService<'a, C> {
    client: &'a C,
}

impl<'a, C> ManagersService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "async")]
impl<'a> ManagersService<'a, Client> {
    /// `GET /redfish/v1/Managers`
    pub async fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Managers"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Managers/{id}`
    pub async fn get(&self, manager_id: &str) -> Result<Manager> {
        let url = self.client.redfish_url(&["Managers", manager_id])?;
        self.client.get_json(url).await
    }

    /// Reset a manager.
    ///
    /// `POST /redfish/v1/Managers/{id}/Actions/Manager.Reset`
    pub async fn reset(
        &self,
        manager_id: &str,
        reset_type: ResetType,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Managers", manager_id, "Actions", "Manager.Reset"])?;

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
impl<'a> ManagersService<'a, BlockingClient> {
    /// `GET /redfish/v1/Managers`
    pub fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Managers"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/Managers/{id}`
    pub fn get(&self, manager_id: &str) -> Result<Manager> {
        let url = self.client.redfish_url(&["Managers", manager_id])?;
        self.client.get_json(url)
    }

    /// Reset a manager.
    ///
    /// `POST /redfish/v1/Managers/{id}/Actions/Manager.Reset`
    pub fn reset(
        &self,
        manager_id: &str,
        reset_type: ResetType,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Managers", manager_id, "Actions", "Manager.Reset"])?;

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
