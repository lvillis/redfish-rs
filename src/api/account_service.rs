use http::Method;

use crate::api::ActionResponse;
use crate::types::{
    Account, AccountCreateRequest, AccountService, AccountUpdateRequest, Collection, OdataId, Role,
};
use crate::{Error, Result};

#[cfg(feature = "_blocking")]
use crate::BlockingClient;
#[cfg(feature = "_async")]
use crate::Client;

/// Access `AccountService`.
#[derive(Debug, Clone, Copy)]
pub struct AccountServiceService<'a, C> {
    client: &'a C,
}

impl<'a, C> AccountServiceService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "_async")]
impl<'a> AccountServiceService<'a, Client> {
    /// `GET /redfish/v1/AccountService`
    pub async fn get(&self) -> Result<AccountService> {
        let url = self.client.redfish_url(&["AccountService"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/AccountService/Accounts`
    pub async fn list_accounts(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["AccountService", "Accounts"])?;
        self.client.get_json(url).await
    }

    /// Convenience helper: follow `Members@odata.nextLink` and return all account members.
    pub async fn list_accounts_all(&self) -> Result<Vec<OdataId>> {
        self.client
            .collect_all_collection_members("/redfish/v1/AccountService/Accounts")
            .await
    }

    /// `GET /redfish/v1/AccountService/Accounts/{id}`
    pub async fn get_account(&self, account_id: &str) -> Result<Account> {
        let url = self
            .client
            .redfish_url(&["AccountService", "Accounts", account_id])?;
        self.client.get_json(url).await
    }

    /// `POST /redfish/v1/AccountService/Accounts`
    pub async fn create_account(
        &self,
        req: &AccountCreateRequest,
    ) -> Result<ActionResponse<Account>> {
        let url = self.client.redfish_url(&["AccountService", "Accounts"])?;

        let raw = self.client.post_json_raw(url.clone(), req).await?;
        ActionResponse::<Account>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `PATCH /redfish/v1/AccountService/Accounts/{id}`
    pub async fn update_account(
        &self,
        account_id: &str,
        req: &AccountUpdateRequest,
    ) -> Result<ActionResponse<Account>> {
        if req.is_empty() {
            return Err(Error::invalid_config("AccountUpdateRequest is empty"));
        }

        let url = self
            .client
            .redfish_url(&["AccountService", "Accounts", account_id])?;

        let raw = self.client.patch_json_raw(url.clone(), req).await?;
        ActionResponse::<Account>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `DELETE /redfish/v1/AccountService/Accounts/{id}`
    pub async fn delete_account(&self, account_id: &str) -> Result<()> {
        let url = self
            .client
            .redfish_url(&["AccountService", "Accounts", account_id])?;
        self.client.delete_raw(url).await?;
        Ok(())
    }

    /// `GET /redfish/v1/AccountService/Roles`
    pub async fn list_roles(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["AccountService", "Roles"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/AccountService/Roles/{id}`
    pub async fn get_role(&self, role_id: &str) -> Result<Role> {
        let url = self
            .client
            .redfish_url(&["AccountService", "Roles", role_id])?;
        self.client.get_json(url).await
    }
}

#[cfg(feature = "_blocking")]
impl<'a> AccountServiceService<'a, BlockingClient> {
    /// `GET /redfish/v1/AccountService`
    pub fn get(&self) -> Result<AccountService> {
        let url = self.client.redfish_url(&["AccountService"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/AccountService/Accounts`
    pub fn list_accounts(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["AccountService", "Accounts"])?;
        self.client.get_json(url)
    }

    /// Convenience helper: follow `Members@odata.nextLink` and return all account members.
    pub fn list_accounts_all(&self) -> Result<Vec<OdataId>> {
        self.client
            .collect_all_collection_members("/redfish/v1/AccountService/Accounts")
    }

    /// `GET /redfish/v1/AccountService/Accounts/{id}`
    pub fn get_account(&self, account_id: &str) -> Result<Account> {
        let url = self
            .client
            .redfish_url(&["AccountService", "Accounts", account_id])?;
        self.client.get_json(url)
    }

    /// `POST /redfish/v1/AccountService/Accounts`
    pub fn create_account(&self, req: &AccountCreateRequest) -> Result<ActionResponse<Account>> {
        let url = self.client.redfish_url(&["AccountService", "Accounts"])?;

        let raw = self.client.post_json_raw(url.clone(), req)?;
        ActionResponse::<Account>::from_raw_json(
            Method::POST,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `PATCH /redfish/v1/AccountService/Accounts/{id}`
    pub fn update_account(
        &self,
        account_id: &str,
        req: &AccountUpdateRequest,
    ) -> Result<ActionResponse<Account>> {
        if req.is_empty() {
            return Err(Error::invalid_config("AccountUpdateRequest is empty"));
        }

        let url = self
            .client
            .redfish_url(&["AccountService", "Accounts", account_id])?;

        let raw = self.client.patch_json_raw(url.clone(), req)?;
        ActionResponse::<Account>::from_raw_json(
            Method::PATCH,
            &url,
            raw,
            self.client.body_snippet_limit(),
        )
    }

    /// `DELETE /redfish/v1/AccountService/Accounts/{id}`
    pub fn delete_account(&self, account_id: &str) -> Result<()> {
        let url = self
            .client
            .redfish_url(&["AccountService", "Accounts", account_id])?;
        self.client.delete_raw(url)?;
        Ok(())
    }

    /// `GET /redfish/v1/AccountService/Roles`
    pub fn list_roles(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["AccountService", "Roles"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/AccountService/Roles/{id}`
    pub fn get_role(&self, role_id: &str) -> Result<Role> {
        let url = self
            .client
            .redfish_url(&["AccountService", "Roles", role_id])?;
        self.client.get_json(url)
    }
}
