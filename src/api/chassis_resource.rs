use http::Method;

use crate::Result;
use crate::api::ActionResponse;
use crate::types::{
    Chassis, Collection, Drive, LogEntry, LogService, ODataQuery, OdataId, Power, ResetType,
    Sensor, Thermal, actions::ResetRequest,
};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access a specific `Chassis` and its common sub-resources.
#[derive(Debug, Clone)]
pub struct ChassisResourceService<'a, C> {
    client: &'a C,
    chassis_id: String,
}

impl<'a, C> ChassisResourceService<'a, C> {
    pub(crate) fn new(client: &'a C, chassis_id: impl Into<String>) -> Self {
        Self {
            client,
            chassis_id: chassis_id.into(),
        }
    }

    fn id(&self) -> &str {
        &self.chassis_id
    }
}

#[cfg(feature = "async")]
impl<'a> ChassisResourceService<'a, Client> {
    /// `GET /redfish/v1/Chassis/{id}`
    pub async fn get(&self) -> Result<Chassis> {
        let url = self.client.redfish_url(&["Chassis", self.id()])?;
        self.client.get_json(url).await
    }

    /// Reset a chassis.
    ///
    /// `POST /redfish/v1/Chassis/{id}/Actions/Chassis.Reset`
    pub async fn reset(&self, reset_type: ResetType) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Actions", "Chassis.Reset"])?;
        let req = ResetRequest { reset_type };
        let raw = self.client.post_json_raw(url.clone(), &req).await?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `GET /redfish/v1/Chassis/{id}/Power`
    pub async fn get_power(&self) -> Result<Power> {
        let url = self.client.redfish_url(&["Chassis", self.id(), "Power"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Chassis/{id}/Thermal`
    pub async fn get_thermal(&self) -> Result<Thermal> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Thermal"])?;
        self.client.get_json(url).await
    }

    /// List sensors.
    ///
    /// `GET /redfish/v1/Chassis/{id}/Sensors`
    pub async fn list_sensors(&self) -> Result<Collection<Sensor>> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Sensors"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Chassis/{id}/Sensors/{sensor_id}`
    pub async fn get_sensor(&self, sensor_id: &str) -> Result<Sensor> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Sensors", sensor_id])?;
        self.client.get_json(url).await
    }

    /// List drives for this chassis (if supported).
    ///
    /// `GET /redfish/v1/Chassis/{id}/Drives`
    pub async fn list_drives(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Chassis", self.id(), "Drives"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Chassis/{id}/Drives/{drive_id}`
    pub async fn get_drive(&self, drive_id: &str) -> Result<Drive> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Drives", drive_id])?;
        self.client.get_json(url).await
    }

    /// List log services for this chassis (if supported).
    ///
    /// `GET /redfish/v1/Chassis/{id}/LogServices`
    pub async fn list_log_services(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "LogServices"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Chassis/{id}/LogServices/{log_id}`
    pub async fn get_log_service(&self, log_id: &str) -> Result<LogService> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "LogServices", log_id])?;
        self.client.get_json(url).await
    }

    /// List log entries.
    ///
    /// `GET /redfish/v1/Chassis/{id}/LogServices/{log_id}/Entries`
    pub async fn list_log_entries(
        &self,
        log_id: &str,
        query: Option<&ODataQuery>,
    ) -> Result<Collection<LogEntry>> {
        let mut url =
            self.client
                .redfish_url(&["Chassis", self.id(), "LogServices", log_id, "Entries"])?;
        if let Some(q) = query {
            url = q.apply_to_url(url);
        }
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/Chassis/{id}/LogServices/{log_id}/Entries/{entry_id}`
    pub async fn get_log_entry(&self, log_id: &str, entry_id: &str) -> Result<LogEntry> {
        let url = self.client.redfish_url(&[
            "Chassis",
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
    /// `POST /redfish/v1/Chassis/{id}/LogServices/{log_id}/Actions/LogService.ClearLog`
    pub async fn clear_log(&self, log_id: &str) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&[
            "Chassis",
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

#[cfg(feature = "blocking")]
impl<'a> ChassisResourceService<'a, BlockingClient> {
    pub fn get(&self) -> Result<Chassis> {
        let url = self.client.redfish_url(&["Chassis", self.id()])?;
        self.client.get_json(url)
    }

    pub fn reset(&self, reset_type: ResetType) -> Result<ActionResponse<serde_json::Value>> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Actions", "Chassis.Reset"])?;
        let req = ResetRequest { reset_type };
        let raw = self.client.post_json_raw(url.clone(), &req)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    pub fn get_power(&self) -> Result<Power> {
        let url = self.client.redfish_url(&["Chassis", self.id(), "Power"])?;
        self.client.get_json(url)
    }

    pub fn get_thermal(&self) -> Result<Thermal> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Thermal"])?;
        self.client.get_json(url)
    }

    pub fn list_sensors(&self) -> Result<Collection<Sensor>> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Sensors"])?;
        self.client.get_json(url)
    }

    pub fn get_sensor(&self, sensor_id: &str) -> Result<Sensor> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Sensors", sensor_id])?;
        self.client.get_json(url)
    }

    pub fn list_drives(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["Chassis", self.id(), "Drives"])?;
        self.client.get_json(url)
    }

    pub fn get_drive(&self, drive_id: &str) -> Result<Drive> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "Drives", drive_id])?;
        self.client.get_json(url)
    }

    pub fn list_log_services(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "LogServices"])?;
        self.client.get_json(url)
    }

    pub fn get_log_service(&self, log_id: &str) -> Result<LogService> {
        let url = self
            .client
            .redfish_url(&["Chassis", self.id(), "LogServices", log_id])?;
        self.client.get_json(url)
    }

    pub fn list_log_entries(
        &self,
        log_id: &str,
        query: Option<&ODataQuery>,
    ) -> Result<Collection<LogEntry>> {
        let mut url =
            self.client
                .redfish_url(&["Chassis", self.id(), "LogServices", log_id, "Entries"])?;
        if let Some(q) = query {
            url = q.apply_to_url(url);
        }
        self.client.get_json(url)
    }

    pub fn get_log_entry(&self, log_id: &str, entry_id: &str) -> Result<LogEntry> {
        let url = self.client.redfish_url(&[
            "Chassis",
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
            "Chassis",
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
