use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::groups::types::Groups;
use crate::beyond_identity::api::roles::types::Roles;
use crate::common::error::BiError;

use super::create::{create_identity, CreateIdentityRequest};
use super::delete::delete_identity;
use super::get::get_identity;
use super::list::list_identities;
use super::list_groups::list_groups;
use super::list_roles::list_roles;
use super::patch::{patch_identity, PatchIdentityRequest};
use super::types::{Identities, Identity};
use crate::beyond_identity::api::common::filter::Filter;

// ====================================
// Identities Service
// ====================================

pub struct IdentityService {
    pub api_client: ApiClient,
}

impl IdentityService {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }
}

// ====================================
// Identities API
// ====================================

pub trait IdentitiesApi {
    async fn create_identity(&self, request: CreateIdentityRequest) -> Result<Identity, BiError>;
    async fn delete_identity(&self, identity_id: &str) -> Result<serde_json::Value, BiError>;
    async fn get_identity(&self, identity_id: &str) -> Result<Identity, BiError>;
    async fn list_identities(&self, filter: Option<Filter>) -> Result<Identities, BiError>;
    async fn list_groups(&self, identity_id: &str) -> Result<Groups, BiError>;
    async fn list_roles(
        &self,
        identity_id: &str,
        resource_server_id: &str,
    ) -> Result<Roles, BiError>;
    async fn patch_identity(
        &self,
        identity_id: &str,
        patch_request: &PatchIdentityRequest,
    ) -> Result<Identity, BiError>;
}

// ====================================
// Identities API Implementation
// ====================================

impl IdentitiesApi for IdentityService {
    async fn create_identity(&self, request: CreateIdentityRequest) -> Result<Identity, BiError> {
        create_identity(self, &request).await
    }

    async fn delete_identity(&self, identity_id: &str) -> Result<serde_json::Value, BiError> {
        delete_identity(self, identity_id).await
    }

    async fn get_identity(&self, identity_id: &str) -> Result<Identity, BiError> {
        get_identity(self, identity_id).await
    }

    async fn list_identities(&self, filter: Option<Filter>) -> Result<Identities, BiError> {
        list_identities(self, filter).await
    }

    async fn list_groups(&self, identity_id: &str) -> Result<Groups, BiError> {
        list_groups(self, identity_id).await
    }

    async fn list_roles(
        &self,
        identity_id: &str,
        resource_server_id: &str,
    ) -> Result<Roles, BiError> {
        list_roles(self, identity_id, resource_server_id).await
    }

    async fn patch_identity(
        &self,
        identity_id: &str,
        patch_request: &PatchIdentityRequest,
    ) -> Result<Identity, BiError> {
        patch_identity(self, identity_id, patch_request).await
    }
}
