use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{Collection, OdataId, SimpleUpdateRequest, SoftwareInventory, UpdateService};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access `UpdateService`.
#[derive(Debug, Clone, Copy)]
pub struct UpdateServiceService<'a, C> {
    client: &'a C,
}

impl<'a, C> UpdateServiceService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "async")]
impl<'a> UpdateServiceService<'a, Client> {
    /// `GET /redfish/v1/UpdateService`
    pub async fn get(&self) -> Result<UpdateService> {
        let url = self.client.redfish_url(&["UpdateService"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/UpdateService/FirmwareInventory`
    pub async fn firmware_inventory(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "FirmwareInventory"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/UpdateService/FirmwareInventory/{id}`
    pub async fn get_firmware_inventory(&self, inventory_id: &str) -> Result<SoftwareInventory> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "FirmwareInventory", inventory_id])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/UpdateService/SoftwareInventory`
    pub async fn software_inventory(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "SoftwareInventory"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/UpdateService/SoftwareInventory/{id}`
    pub async fn get_software_inventory(&self, inventory_id: &str) -> Result<SoftwareInventory> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "SoftwareInventory", inventory_id])?;
        self.client.get_json(url).await
    }

    /// Perform a `SimpleUpdate` action.
    ///
    /// `POST /redfish/v1/UpdateService/Actions/UpdateService.SimpleUpdate`
    pub async fn simple_update(
        &self,
        req: &SimpleUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url =
            self.client
                .redfish_url(&["UpdateService", "Actions", "UpdateService.SimpleUpdate"])?;

        let raw = self.client.post_json_raw(url.clone(), req).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}

#[cfg(feature = "blocking")]
impl<'a> UpdateServiceService<'a, BlockingClient> {
    /// `GET /redfish/v1/UpdateService`
    pub fn get(&self) -> Result<UpdateService> {
        let url = self.client.redfish_url(&["UpdateService"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/UpdateService/FirmwareInventory`
    pub fn firmware_inventory(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "FirmwareInventory"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/UpdateService/FirmwareInventory/{id}`
    pub fn get_firmware_inventory(&self, inventory_id: &str) -> Result<SoftwareInventory> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "FirmwareInventory", inventory_id])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/UpdateService/SoftwareInventory`
    pub fn software_inventory(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "SoftwareInventory"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/UpdateService/SoftwareInventory/{id}`
    pub fn get_software_inventory(&self, inventory_id: &str) -> Result<SoftwareInventory> {
        let url = self
            .client
            .redfish_url(&["UpdateService", "SoftwareInventory", inventory_id])?;
        self.client.get_json(url)
    }

    /// Perform a `SimpleUpdate` action.
    ///
    /// `POST /redfish/v1/UpdateService/Actions/UpdateService.SimpleUpdate`
    pub fn simple_update(
        &self,
        req: &SimpleUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url =
            self.client
                .redfish_url(&["UpdateService", "Actions", "UpdateService.SimpleUpdate"])?;

        let raw = self.client.post_json_raw(url.clone(), req)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}
