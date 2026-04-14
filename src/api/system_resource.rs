use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{
    Bios, BiosSettings, BiosSettingsUpdateRequest, Collection, ComputerSystem,
    ComputerSystemUpdateRequest, Drive, EthernetInterface, EthernetInterfaceUpdateRequest,
    LogEntry, LogService, ODataQuery, OdataId, Storage,
};

#[cfg(feature = "_blocking")]
use crate::BlockingClient;
#[cfg(feature = "_async")]
use crate::Client;

/// Access a specific `ComputerSystem` and its common sub-resources.
///
/// This helper is a convenience wrapper around the `Systems` collection.
#[derive(Debug, Clone)]
pub struct SystemResourceService<'a, C> {
    client: &'a C,
    system_id: String,
}

impl<'a, C> SystemResourceService<'a, C> {
    pub(crate) fn new(client: &'a C, system_id: impl Into<String>) -> Self {
        Self {
            client,
            system_id: system_id.into(),
        }
    }

    fn id(&self) -> &str {
        &self.system_id
    }
}

#[cfg(feature = "_async")]
impl<'a> SystemResourceService<'a, Client> {
    /// `GET /redfish/v1/Systems/{id}`
    pub async fn get(&self) -> Result<ComputerSystem> {
        let url = self.client.redfish_url(&["Systems", self.id()])?;
        self.client.get_json(url).await
    }

    /// Patch a `ComputerSystem`.
    ///
    /// Typical path: `PATCH /redfish/v1/Systems/{id}`
    pub async fn patch(
        &self,
        request: &ComputerSystemUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&["Systems", self.id()])?;
        let raw = self.client.patch_json_raw(url.clone(), request).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `GET /redfish/v1/Systems/{id}/Bios`
    pub async fn get_bios(&self) -> Result<Bios> {
        let url = self.client.redfish_url(&["Systems", self.id(), "Bios"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}/Bios/Settings`
    pub async fn get_bios_settings(&self) -> Result<BiosSettings> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Bios", "Settings"])?;
        self.client.get_json(url).await
    }

    /// Patch BIOS settings.
    ///
    /// Typical path: `PATCH /redfish/v1/Systems/{id}/Bios/Settings`
    pub async fn patch_bios_settings(
        &self,
        request: &BiosSettingsUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Bios", "Settings"])?;
        let raw = self.client.patch_json_raw(url.clone(), request).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// List system Ethernet interfaces.
    ///
    /// `GET /redfish/v1/Systems/{id}/EthernetInterfaces`
    pub async fn list_ethernet_interfaces(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "EthernetInterfaces"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}/EthernetInterfaces/{eth_id}`
    pub async fn get_ethernet_interface(&self, eth_id: &str) -> Result<EthernetInterface> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "EthernetInterfaces", eth_id])?;
        self.client.get_json(url).await
    }

    /// Patch an Ethernet interface.
    pub async fn patch_ethernet_interface(
        &self,
        eth_id: &str,
        request: &EthernetInterfaceUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "EthernetInterfaces", eth_id])?;
        let raw = self.client.patch_json_raw(url.clone(), request).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// List system storages.
    ///
    /// `GET /redfish/v1/Systems/{id}/Storage`
    pub async fn list_storage(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Storage"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}/Storage/{storage_id}`
    pub async fn get_storage(&self, storage_id: &str) -> Result<Storage> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Storage", storage_id])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}/Storage/{storage_id}/Drives/{drive_id}`
    pub async fn get_drive(&self, storage_id: &str, drive_id: &str) -> Result<Drive> {
        let url = self.client.redfish_url(&[
            "Systems",
            self.id(),
            "Storage",
            storage_id,
            "Drives",
            drive_id,
        ])?;
        self.client.get_json(url).await
    }

    /// List log services for this system.
    ///
    /// `GET /redfish/v1/Systems/{id}/LogServices`
    pub async fn list_log_services(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "LogServices"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}/LogServices/{log_id}`
    pub async fn get_log_service(&self, log_id: &str) -> Result<LogService> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "LogServices", log_id])?;
        self.client.get_json(url).await
    }

    /// List log entries.
    ///
    /// `GET /redfish/v1/Systems/{id}/LogServices/{log_id}/Entries`
    pub async fn list_log_entries(
        &self,
        log_id: &str,
        query: Option<&ODataQuery>,
    ) -> Result<Collection<LogEntry>> {
        let mut url =
            self.client
                .redfish_url(&["Systems", self.id(), "LogServices", log_id, "Entries"])?;
        if let Some(q) = query {
            url = q.apply_to_url(url);
        }
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Systems/{id}/LogServices/{log_id}/Entries/{entry_id}`
    pub async fn get_log_entry(&self, log_id: &str, entry_id: &str) -> Result<LogEntry> {
        let url = self.client.redfish_url(&[
            "Systems",
            self.id(),
            "LogServices",
            log_id,
            "Entries",
            entry_id,
        ])?;
        self.client.get_json(url).await
    }

    /// Clear a log.
    ///
    /// `POST /redfish/v1/Systems/{id}/LogServices/{log_id}/Actions/LogService.ClearLog`
    pub async fn clear_log(&self, log_id: &str) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&[
            "Systems",
            self.id(),
            "LogServices",
            log_id,
            "Actions",
            "LogService.ClearLog",
        ])?;
        let body = serde_json::json!({});
        let raw = self.client.post_json_raw(url.clone(), &body).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}

#[cfg(feature = "_blocking")]
impl<'a> SystemResourceService<'a, BlockingClient> {
    pub fn get(&self) -> Result<ComputerSystem> {
        let url = self.client.redfish_url(&["Systems", self.id()])?;
        self.client.get_json(url)
    }

    pub fn patch(
        &self,
        request: &ComputerSystemUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&["Systems", self.id()])?;
        let raw = self.client.patch_json_raw(url.clone(), request)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    pub fn get_bios(&self) -> Result<Bios> {
        let url = self.client.redfish_url(&["Systems", self.id(), "Bios"])?;
        self.client.get_json(url)
    }

    pub fn get_bios_settings(&self) -> Result<BiosSettings> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Bios", "Settings"])?;
        self.client.get_json(url)
    }

    pub fn patch_bios_settings(
        &self,
        request: &BiosSettingsUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Bios", "Settings"])?;
        let raw = self.client.patch_json_raw(url.clone(), request)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    pub fn list_ethernet_interfaces(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "EthernetInterfaces"])?;
        self.client.get_json(url)
    }

    pub fn get_ethernet_interface(&self, eth_id: &str) -> Result<EthernetInterface> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "EthernetInterfaces", eth_id])?;
        self.client.get_json(url)
    }

    pub fn patch_ethernet_interface(
        &self,
        eth_id: &str,
        request: &EthernetInterfaceUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "EthernetInterfaces", eth_id])?;
        let raw = self.client.patch_json_raw(url.clone(), request)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    pub fn list_storage(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Storage"])?;
        self.client.get_json(url)
    }

    pub fn get_storage(&self, storage_id: &str) -> Result<Storage> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "Storage", storage_id])?;
        self.client.get_json(url)
    }

    pub fn get_drive(&self, storage_id: &str, drive_id: &str) -> Result<Drive> {
        let url = self.client.redfish_url(&[
            "Systems",
            self.id(),
            "Storage",
            storage_id,
            "Drives",
            drive_id,
        ])?;
        self.client.get_json(url)
    }

    pub fn list_log_services(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "LogServices"])?;
        self.client.get_json(url)
    }

    pub fn get_log_service(&self, log_id: &str) -> Result<LogService> {
        let url = self
            .client
            .redfish_url(&["Systems", self.id(), "LogServices", log_id])?;
        self.client.get_json(url)
    }

    pub fn list_log_entries(
        &self,
        log_id: &str,
        query: Option<&ODataQuery>,
    ) -> Result<Collection<LogEntry>> {
        let mut url =
            self.client
                .redfish_url(&["Systems", self.id(), "LogServices", log_id, "Entries"])?;
        if let Some(q) = query {
            url = q.apply_to_url(url);
        }
        self.client.get_json(url)
    }

    pub fn get_log_entry(&self, log_id: &str, entry_id: &str) -> Result<LogEntry> {
        let url = self.client.redfish_url(&[
            "Systems",
            self.id(),
            "LogServices",
            log_id,
            "Entries",
            entry_id,
        ])?;
        self.client.get_json(url)
    }

    pub fn clear_log(&self, log_id: &str) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&[
            "Systems",
            self.id(),
            "LogServices",
            log_id,
            "Actions",
            "LogService.ClearLog",
        ])?;
        let body = serde_json::json!({});
        let raw = self.client.post_json_raw(url.clone(), &body)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}
