use http::header::LOCATION;
use serde::Serialize;

use crate::auth::X_AUTH_TOKEN;
use crate::types::{Collection, OdataId, Session};
use crate::{Error, Result, SessionToken};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Result of a successful session login.
#[derive(Debug, Clone)]
pub struct SessionLogin {
    /// Session token (to be used as `X-Auth-Token`).
    pub token: SessionToken,
    /// Session resource URI (from `Location` header).
    pub location: String,
    /// Parsed session resource, if the server returned a JSON body.
    pub session: Option<Session>,
}

/// Access `SessionService`.
#[derive(Debug, Clone, Copy)]
pub struct SessionsService<'a, C> {
    client: &'a C,
}

impl<'a, C> SessionsService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
struct SessionCreateRequest<'a> {
    #[serde(rename = "UserName")]
    user_name: &'a str,
    #[serde(rename = "Password")]
    password: &'a str,
}

#[cfg(feature = "async")]
impl<'a> SessionsService<'a, Client> {
    /// List sessions.
    ///
    /// `GET /redfish/v1/SessionService/Sessions`
    pub async fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["SessionService", "Sessions"])?;
        self.client.get_json(url).await
    }

    /// Get a session resource by id.
    ///
    /// `GET /redfish/v1/SessionService/Sessions/{id}`
    pub async fn get(&self, session_id: &str) -> Result<Session> {
        let url = self
            .client
            .redfish_url(&["SessionService", "Sessions", session_id])?;
        self.client.get_json(url).await
    }

    /// Create a Redfish session.
    ///
    /// `POST /redfish/v1/SessionService/Sessions`
    pub async fn create(&self, username: &str, password: &str) -> Result<SessionLogin> {
        let url = self.client.redfish_url(&["SessionService", "Sessions"])?;

        let req = SessionCreateRequest {
            user_name: username,
            password,
        };

        let response = self.client.post_json_raw(url, &req).await?;

        let token = response
            .headers
            .get(X_AUTH_TOKEN.clone())
            .and_then(|v| v.to_str().ok())
            .map(SessionToken::new)
            .ok_or_else(|| Error::auth("Missing X-Auth-Token in session response"))?;

        let location = response
            .headers
            .get(LOCATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .ok_or_else(|| Error::auth("Missing Location in session response"))?;

        let session = if response.body.is_empty() {
            None
        } else {
            serde_json::from_slice::<Session>(&response.body).ok()
        };

        Ok(SessionLogin {
            token,
            location,
            session,
        })
    }

    /// Delete an existing session.
    ///
    /// The `location` is typically obtained from [`SessionsService::create`].
    pub async fn delete(&self, location: &str) -> Result<()> {
        let url = self.client.resolve_uri(location)?;
        self.client.delete_raw(url).await?;
        Ok(())
    }
}

#[cfg(feature = "blocking")]
impl<'a> SessionsService<'a, BlockingClient> {
    /// List sessions.
    ///
    /// `GET /redfish/v1/SessionService/Sessions`
    pub fn list(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["SessionService", "Sessions"])?;
        self.client.get_json(url)
    }

    /// Get a session resource by id.
    ///
    /// `GET /redfish/v1/SessionService/Sessions/{id}`
    pub fn get(&self, session_id: &str) -> Result<Session> {
        let url = self
            .client
            .redfish_url(&["SessionService", "Sessions", session_id])?;
        self.client.get_json(url)
    }

    /// Create a Redfish session.
    ///
    /// `POST /redfish/v1/SessionService/Sessions`
    pub fn create(&self, username: &str, password: &str) -> Result<SessionLogin> {
        let url = self.client.redfish_url(&["SessionService", "Sessions"])?;

        let req = SessionCreateRequest {
            user_name: username,
            password,
        };

        let response = self.client.post_json_raw(url, &req)?;

        let token = response
            .headers
            .get(X_AUTH_TOKEN.clone())
            .and_then(|v| v.to_str().ok())
            .map(SessionToken::new)
            .ok_or_else(|| Error::auth("Missing X-Auth-Token in session response"))?;

        let location = response
            .headers
            .get(LOCATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .ok_or_else(|| Error::auth("Missing Location in session response"))?;

        let session = if response.body.is_empty() {
            None
        } else {
            serde_json::from_slice::<Session>(&response.body).ok()
        };

        Ok(SessionLogin {
            token,
            location,
            session,
        })
    }

    /// Delete an existing session.
    ///
    /// The `location` is typically obtained from [`SessionsService::create`].
    pub fn delete(&self, location: &str) -> Result<()> {
        let url = self.client.resolve_uri(location)?;
        self.client.delete_raw(url)?;
        Ok(())
    }
}
