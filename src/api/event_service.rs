use http::Method;

use crate::api::ActionResponse;
use crate::types::{
    Collection, EventService, EventSubscription, EventSubscriptionCreateRequest,
    EventSubscriptionUpdateRequest, OdataId, SubmitTestEventRequest,
};
use crate::{Error, Result};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access `EventService`.
#[derive(Debug, Clone, Copy)]
pub struct EventServiceService<'a, C> {
    client: &'a C,
}

impl<'a, C> EventServiceService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "async")]
impl<'a> EventServiceService<'a, Client> {
    /// `GET /redfish/v1/EventService`
    pub async fn get(&self) -> Result<EventService> {
        let url = self.client.redfish_url(&["EventService"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/EventService/Subscriptions`
    pub async fn list_subscriptions(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions"])?;
        self.client.get_json(url).await
    }

    /// Convenience helper: follow `Members@odata.nextLink` and return all subscription members.
    pub async fn list_subscriptions_all(&self) -> Result<Vec<OdataId>> {
        self.client
            .collect_all_collection_members("/redfish/v1/EventService/Subscriptions")
            .await
    }

    /// `GET /redfish/v1/EventService/Subscriptions/{id}`
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<EventSubscription> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions", subscription_id])?;
        self.client.get_json(url).await
    }

    /// `POST /redfish/v1/EventService/Subscriptions`
    pub async fn create_subscription(
        &self,
        req: &EventSubscriptionCreateRequest,
    ) -> Result<ActionResponse<EventSubscription>> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions"])?;
        let raw = self.client.post_json_raw(url.clone(), req).await?;
        ActionResponse::<EventSubscription>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `PATCH /redfish/v1/EventService/Subscriptions/{id}`
    pub async fn update_subscription(
        &self,
        subscription_id: &str,
        req: &EventSubscriptionUpdateRequest,
    ) -> Result<ActionResponse<EventSubscription>> {
        if req.is_empty() {
            return Err(Error::invalid_config(
                "EventSubscriptionUpdateRequest is empty",
            ));
        }

        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions", subscription_id])?;
        let raw = self.client.patch_json_raw(url.clone(), req).await?;
        ActionResponse::<EventSubscription>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `DELETE /redfish/v1/EventService/Subscriptions/{id}`
    pub async fn delete_subscription(&self, subscription_id: &str) -> Result<()> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions", subscription_id])?;
        self.client.delete_raw(url).await?;
        Ok(())
    }

    /// Submit a test event.
    ///
    /// `POST /redfish/v1/EventService/Actions/EventService.SubmitTestEvent`
    pub async fn submit_test_event(
        &self,
        req: &SubmitTestEventRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&[
            "EventService",
            "Actions",
            "EventService.SubmitTestEvent",
        ])?;

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
impl<'a> EventServiceService<'a, BlockingClient> {
    /// `GET /redfish/v1/EventService`
    pub fn get(&self) -> Result<EventService> {
        let url = self.client.redfish_url(&["EventService"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/EventService/Subscriptions`
    pub fn list_subscriptions(&self) -> Result<Collection<OdataId>> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions"])?;
        self.client.get_json(url)
    }

    /// Convenience helper: follow `Members@odata.nextLink` and return all subscription members.
    pub fn list_subscriptions_all(&self) -> Result<Vec<OdataId>> {
        self.client
            .collect_all_collection_members("/redfish/v1/EventService/Subscriptions")
    }

    /// `GET /redfish/v1/EventService/Subscriptions/{id}`
    pub fn get_subscription(&self, subscription_id: &str) -> Result<EventSubscription> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions", subscription_id])?;
        self.client.get_json(url)
    }

    /// `POST /redfish/v1/EventService/Subscriptions`
    pub fn create_subscription(
        &self,
        req: &EventSubscriptionCreateRequest,
    ) -> Result<ActionResponse<EventSubscription>> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions"])?;
        let raw = self.client.post_json_raw(url.clone(), req)?;
        ActionResponse::<EventSubscription>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `PATCH /redfish/v1/EventService/Subscriptions/{id}`
    pub fn update_subscription(
        &self,
        subscription_id: &str,
        req: &EventSubscriptionUpdateRequest,
    ) -> Result<ActionResponse<EventSubscription>> {
        if req.is_empty() {
            return Err(Error::invalid_config(
                "EventSubscriptionUpdateRequest is empty",
            ));
        }

        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions", subscription_id])?;
        let raw = self.client.patch_json_raw(url.clone(), req)?;
        ActionResponse::<EventSubscription>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `DELETE /redfish/v1/EventService/Subscriptions/{id}`
    pub fn delete_subscription(&self, subscription_id: &str) -> Result<()> {
        let url = self
            .client
            .redfish_url(&["EventService", "Subscriptions", subscription_id])?;
        self.client.delete_raw(url)?;
        Ok(())
    }

    /// Submit a test event.
    ///
    /// `POST /redfish/v1/EventService/Actions/EventService.SubmitTestEvent`
    pub fn submit_test_event(
        &self,
        req: &SubmitTestEventRequest,
    ) -> Result<ActionResponse<serde_json::Value>> {
        let url = self.client.redfish_url(&[
            "EventService",
            "Actions",
            "EventService.SubmitTestEvent",
        ])?;

        let raw = self.client.post_json_raw(url.clone(), req)?;
        ActionResponse::<serde_json::Value>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }
}
