use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{
    Collection, EthernetInterface, EthernetInterfaceUpdateRequest, LogEntry, LogService, Manager,
    ManagerNetworkProtocol, ManagerNetworkProtocolUpdateRequest, ODataQuery, OdataId, ResetType,
    actions::ResetRequest,
};

#[cfg(feature = "_blocking")]
use crate::BlockingClient;
#[cfg(feature = "_async")]
use crate::Client;

/// Access a specific `Manager` and its common sub-resources.
#[derive(Debug, Clone)]
pub struct ManagerResourceService<'a, C> {
    client: &'a C,
    manager_id: String,
}

impl<'a, C> ManagerResourceService<'a, C> {
    pub(crate) fn new(client: &'a C, manager_id: impl Into<String>) -> Self {
        Self {
            client,
            manager_id: manager_id.into(),
        }
    }

    fn id(&self) -> &str {
        &self.manager_id
    }
}

#[cfg(feature = "_async")]
impl<'a> ManagerResourceService<'a, Client> {
    /// `GET /redfish/v1/Managers/{id}`
    pub async fn get(&self) -> Result<Manager> {
        let url = self.client.redfish_url(&["Managers", self.id()])?;
        self.client.get_json(url).await
    }

    /// Reset a manager.
    ///
    /// `POST /redfish/v1/Managers/{id}/Actions/Manager.Reset`
    pub async fn reset(&self, reset_type: ResetType) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "Actions", "Manager.Reset"])?;
        let req = ResetRequest { reset_type };
        let raw = self.client.post_json_raw(url.clone(), &req).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// List manager Ethernet interfaces.
    ///
    /// `GET /redfish/v1/Managers/{id}/EthernetInterfaces`
    pub async fn list_ethernet_interfaces(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "EthernetInterfaces"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Managers/{id}/EthernetInterfaces/{eth_id}`
    pub async fn get_ethernet_interface(&self, eth_id: &str) -> Result<EthernetInterface> {
        let url =
            self.client
                .redfish_url(&["Managers", self.id(), "EthernetInterfaces", eth_id])?;
        self.client.get_json(url).await
    }

    /// Patch an Ethernet interface.
    pub async fn patch_ethernet_interface(
        &self,
        eth_id: &str,
        request: &EthernetInterfaceUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url =
            self.client
                .redfish_url(&["Managers", self.id(), "EthernetInterfaces", eth_id])?;
        let raw = self.client.patch_json_raw(url.clone(), request).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// List log services for this manager.
    ///
    /// `GET /redfish/v1/Managers/{id}/LogServices`
    pub async fn list_log_services(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "LogServices"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Managers/{id}/LogServices/{log_id}`
    pub async fn get_log_service(&self, log_id: &str) -> Result<LogService> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "LogServices", log_id])?;
        self.client.get_json(url).await
    }

    /// List log entries.
    ///
    /// `GET /redfish/v1/Managers/{id}/LogServices/{log_id}/Entries`
    pub async fn list_log_entries(
        &self,
        log_id: &str,
        query: Option<&ODataQuery>,
    ) -> Result<Collection<LogEntry>> {
        let mut url =
            self.client
                .redfish_url(&["Managers", self.id(), "LogServices", log_id, "Entries"])?;
        if let Some(q) = query {
            url = q.apply_to_url(url);
        }
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Managers/{id}/LogServices/{log_id}/Entries/{entry_id}`
    pub async fn get_log_entry(&self, log_id: &str, entry_id: &str) -> Result<LogEntry> {
        let url = self.client.redfish_url(&[
            "Managers",
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
    /// `POST /redfish/v1/Managers/{id}/LogServices/{log_id}/Actions/LogService.ClearLog`
    pub async fn clear_log(&self, log_id: &str) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&[
            "Managers",
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

    /// `GET /redfish/v1/Managers/{id}/NetworkProtocol`
    pub async fn get_network_protocol(&self) -> Result<ManagerNetworkProtocol> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "NetworkProtocol"])?;
        self.client.get_json(url).await
    }

    /// Patch manager network protocol.
    ///
    /// `PATCH /redfish/v1/Managers/{id}/NetworkProtocol`
    pub async fn patch_network_protocol(
        &self,
        request: &ManagerNetworkProtocolUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "NetworkProtocol"])?;
        let raw = self.client.patch_json_raw(url.clone(), request).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}

#[cfg(feature = "_blocking")]
impl<'a> ManagerResourceService<'a, BlockingClient> {
    pub fn get(&self) -> Result<Manager> {
        let url = self.client.redfish_url(&["Managers", self.id()])?;
        self.client.get_json(url)
    }

    pub fn reset(&self, reset_type: ResetType) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "Actions", "Manager.Reset"])?;
        let req = ResetRequest { reset_type };
        let raw = self.client.post_json_raw(url.clone(), &req)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    pub fn list_ethernet_interfaces(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "EthernetInterfaces"])?;
        self.client.get_json(url)
    }

    pub fn get_ethernet_interface(&self, eth_id: &str) -> Result<EthernetInterface> {
        let url =
            self.client
                .redfish_url(&["Managers", self.id(), "EthernetInterfaces", eth_id])?;
        self.client.get_json(url)
    }

    pub fn patch_ethernet_interface(
        &self,
        eth_id: &str,
        request: &EthernetInterfaceUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url =
            self.client
                .redfish_url(&["Managers", self.id(), "EthernetInterfaces", eth_id])?;
        let raw = self.client.patch_json_raw(url.clone(), request)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    pub fn list_log_services(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "LogServices"])?;
        self.client.get_json(url)
    }

    pub fn get_log_service(&self, log_id: &str) -> Result<LogService> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "LogServices", log_id])?;
        self.client.get_json(url)
    }

    pub fn list_log_entries(
        &self,
        log_id: &str,
        query: Option<&ODataQuery>,
    ) -> Result<Collection<LogEntry>> {
        let mut url =
            self.client
                .redfish_url(&["Managers", self.id(), "LogServices", log_id, "Entries"])?;
        if let Some(q) = query {
            url = q.apply_to_url(url);
        }
        self.client.get_json(url)
    }

    pub fn get_log_entry(&self, log_id: &str, entry_id: &str) -> Result<LogEntry> {
        let url = self.client.redfish_url(&[
            "Managers",
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
            "Managers",
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

    pub fn get_network_protocol(&self) -> Result<ManagerNetworkProtocol> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "NetworkProtocol"])?;
        self.client.get_json(url)
    }

    pub fn patch_network_protocol(
        &self,
        request: &ManagerNetworkProtocolUpdateRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Managers", self.id(), "NetworkProtocol"])?;
        let raw = self.client.patch_json_raw(url.clone(), request)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}
