use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{OdataId, ResourceStatus};

/// Manager account (`ManagerAccount` resource).
///
/// Typically a member of `/redfish/v1/AccountService/Accounts`.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    #[serde(rename = "@odata.id", default)]
    pub odata_id: Option<String>,

    #[serde(rename = "@odata.type", default)]
    pub odata_type: Option<String>,

    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Description", default)]
    pub description: Option<String>,

    #[serde(rename = "UserName", default)]
    pub user_name: Option<String>,

    #[serde(rename = "RoleId", default)]
    pub role_id: Option<String>,

    #[serde(rename = "Enabled", default)]
    pub enabled: Option<bool>,

    #[serde(rename = "Locked", default)]
    pub locked: Option<bool>,

    #[serde(rename = "Links", default)]
    pub links: Option<AccountLinks>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountLinks {
    #[serde(rename = "Role", default)]
    pub role: Option<OdataId>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Request body for `POST /AccountService/Accounts`.
#[derive(Clone, Serialize)]
pub struct AccountCreateRequest {
    #[serde(rename = "UserName")]
    user_name: String,

    #[serde(rename = "Password")]
    password: String,

    #[serde(rename = "RoleId", skip_serializing_if = "Option::is_none")]
    role_id: Option<String>,

    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

impl AccountCreateRequest {
    /// Create a new account creation request.
    ///
    /// The password is stored as plain text because Redfish requires it in the JSON body.
    /// The `Debug` impl redacts it.
    pub fn new(user_name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            user_name: user_name.into(),
            password: password.into(),
            role_id: None,
            enabled: None,
        }
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = Some(role_id.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

impl fmt::Debug for AccountCreateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccountCreateRequest")
            .field("user_name", &self.user_name)
            .field("password", &"<redacted>")
            .field("role_id", &self.role_id)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Request body for `PATCH /AccountService/Accounts/{id}`.
#[derive(Clone, Serialize, Default)]
pub struct AccountUpdateRequest {
    #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
    password: Option<String>,

    #[serde(rename = "RoleId", skip_serializing_if = "Option::is_none")]
    role_id: Option<String>,

    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(rename = "Locked", skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
}

impl AccountUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = Some(role_id.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = Some(locked);
        self
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.password.is_none()
            && self.role_id.is_none()
            && self.enabled.is_none()
            && self.locked.is_none()
    }
}

impl fmt::Debug for AccountUpdateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccountUpdateRequest")
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .field("role_id", &self.role_id)
            .field("enabled", &self.enabled)
            .field("locked", &self.locked)
            .finish()
    }
}
