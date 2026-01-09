use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{Chassis, Collection, OdataId, ResetType, actions::ResetRequest};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access `Chassis` collection and member resources.
#[derive(Debug, Clone, Copy)]
pub struct ChassisService<'a, C> {
    client: &'a C,
}

impl<'a, C> ChassisService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "async")]
impl<'a> ChassisService<'a, Client> {
    /// `GET /redfish/v1/Chassis`
    pub async fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Chassis"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Chassis/{id}`
    pub async fn get(&self, chassis_id: &str) -> Result<Chassis> {
        let url = self.client.redfish_url(&["Chassis", chassis_id])?;
        self.client.get_json(url).await
    }

    /// Reset a chassis.
    ///
    /// `POST /redfish/v1/Chassis/{id}/Actions/Chassis.Reset`
    pub async fn reset(
        &self,
        chassis_id: &str,
        reset_type: ResetType,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Chassis", chassis_id, "Actions", "Chassis.Reset"])?;

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
impl<'a> ChassisService<'a, BlockingClient> {
    /// `GET /redfish/v1/Chassis`
    pub fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Chassis"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/Chassis/{id}`
    pub fn get(&self, chassis_id: &str) -> Result<Chassis> {
        let url = self.client.redfish_url(&["Chassis", chassis_id])?;
        self.client.get_json(url)
    }

    /// Reset a chassis.
    ///
    /// `POST /redfish/v1/Chassis/{id}/Actions/Chassis.Reset`
    pub fn reset(
        &self,
        chassis_id: &str,
        reset_type: ResetType,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Chassis", chassis_id, "Actions", "Chassis.Reset"])?;

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
