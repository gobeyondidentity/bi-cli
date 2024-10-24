use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

use super::types::{
    CreateIdentityRequest, Identities, IdentitiesFieldName, Identity, IdentityDetails,
    PatchIdentityRequest,
};

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::filter::{Filter, FilterFieldName};
use crate::beyond_identity::api::common::request::{send_request, send_request_paginated};
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::beyond_identity::api::groups::types::{GroupDetails, Groups, GroupsFieldName};
use crate::beyond_identity::api::roles::types::{
    RoleDetails, RoleDetailsFieldName, Roles, RolesFieldName,
};
use crate::common::error::BiError;

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
        send_request(
            &self.api_client,
            Method::POST,
            &URLBuilder::build(&self.api_client.tenant_config)
                .api()
                .add_tenant()
                .add_realm()
                .add_path(vec![IdentitiesFieldName::Identities.name()])
                .to_string()?,
            Some(&request),
        )
        .await
        .map(|details| Identity { identity: details })
    }

    async fn delete_identity(&self, identity_id: &str) -> Result<serde_json::Value, BiError> {
        send_request(
            &self.api_client,
            Method::DELETE,
            &URLBuilder::build(&self.api_client.tenant_config)
                .api()
                .add_tenant()
                .add_realm()
                .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
                .to_string()?,
            None::<&()>,
        )
        .await
    }

    async fn get_identity(&self, identity_id: &str) -> Result<Identity, BiError> {
        send_request(
            &self.api_client,
            Method::GET,
            &URLBuilder::build(&self.api_client.tenant_config)
                .api()
                .add_tenant()
                .add_realm()
                .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
                .to_string()?,
            None::<&()>,
        )
        .await
        .map(|details| Identity { identity: details })
    }

    async fn list_identities(&self, filter: Option<Filter>) -> Result<Identities, BiError> {
        let url = URLBuilder::build(&self.api_client.tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name()])
            .add_query_param(
                &FilterFieldName::Filter.name(),
                filter.as_ref().map(|f| f.filter.as_ref()),
            )
            .to_string()?;

        let identities: Vec<IdentityDetails> = send_request_paginated(
            &self.api_client,
            Method::GET,
            &url,
            None::<&()>,
            IdentitiesFieldName::Identities.name(),
        )
        .await?;

        Ok(Identities {
            identities: identities.clone(),
            total_size: identities.len(),
        })
    }

    #[named]
    async fn list_groups(&self, identity_id: &str) -> Result<Groups, BiError> {
        let url = URLBuilder::build(&self.api_client.tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .add_custom_method(&function_name!().to_case(Case::Camel))
            .to_string()?;

        let groups: Vec<GroupDetails> = send_request_paginated(
            &self.api_client,
            Method::GET,
            &url,
            None::<&()>,
            GroupsFieldName::Groups.name(),
        )
        .await?;

        Ok(Groups {
            groups: groups.clone(),
            total_size: groups.len(),
        })
    }

    #[named]
    async fn list_roles(
        &self,
        identity_id: &str,
        resource_server_id: &str,
    ) -> Result<Roles, BiError> {
        let url = URLBuilder::build(&self.api_client.tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .add_custom_method(&function_name!().to_case(Case::Camel))
            .add_query_param(
                &RoleDetailsFieldName::ResourceServerId.name(),
                Some(resource_server_id),
            )
            .to_string()?;

        let roles: Vec<RoleDetails> = send_request_paginated(
            &self.api_client,
            Method::GET,
            &url,
            None::<&()>,
            RolesFieldName::Roles.name(),
        )
        .await?;

        Ok(Roles {
            roles: roles.clone(),
            total_size: roles.len(),
        })
    }

    async fn patch_identity(
        &self,
        identity_id: &str,
        patch_request: &PatchIdentityRequest,
    ) -> Result<Identity, BiError> {
        send_request(
            &self.api_client,
            Method::PATCH,
            &URLBuilder::build(&self.api_client.tenant_config)
                .api()
                .add_tenant()
                .add_realm()
                .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
                .to_string()?,
            Some(patch_request),
        )
        .await
        .map(|details| Identity { identity: details })
    }
}
