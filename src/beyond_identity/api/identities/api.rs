use super::types::{
    CreateIdentityRequest, Identities, IdentitiesFieldName, Identity, PatchIdentityRequest,
};

use crate::beyond_identity::api::common::filter::{Filter, FilterFieldName};
use crate::beyond_identity::api::common::service::IdentitiesService;
use crate::beyond_identity::api::groups::types::{Group, Groups};
use crate::beyond_identity::api::roles::types::{Role, RoleFieldName, Roles};
use crate::common::error::BiError;

use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

// ====================================
// Identities API
// ====================================

pub trait IdentitiesApi {
    async fn create_identity(&self, request: &CreateIdentityRequest) -> Result<Identity, BiError>;
    async fn delete_identity(&self, identity_id: &str) -> Result<serde_json::Value, BiError>;
    async fn get_identity(&self, identity_id: &str) -> Result<Identity, BiError>;
    async fn list_identities(
        &self,
        filter: Option<Filter>,
        limit: Option<usize>,
    ) -> Result<Identities, BiError>;
    async fn list_groups(&self, identity_id: &str, limit: Option<usize>)
        -> Result<Groups, BiError>;
    async fn list_roles(
        &self,
        identity_id: &str,
        resource_server_id: &str,
        limit: Option<usize>,
    ) -> Result<Roles, BiError>;
    async fn patch_identity(&self, request: &PatchIdentityRequest) -> Result<Identity, BiError>;
}

// ====================================
// Identities API Implementation
// ====================================

impl IdentitiesApi for IdentitiesService {
    async fn create_identity(&self, request: &CreateIdentityRequest) -> Result<Identity, BiError> {
        self.api_client
            .send_request(
                Method::POST,
                &self
                    .api_client
                    .build_url()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![IdentitiesFieldName::Identities.name()])
                    .to_string()?,
                Some(request),
            )
            .await
    }

    async fn delete_identity(&self, identity_id: &str) -> Result<serde_json::Value, BiError> {
        self.api_client
            .send_request(
                Method::DELETE,
                &self
                    .api_client
                    .build_url()
                    .await?
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
        self.api_client
            .send_request(
                Method::GET,
                &self
                    .api_client
                    .build_url()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    async fn list_identities(
        &self,
        filter: Option<Filter>,
        limit: Option<usize>,
    ) -> Result<Identities, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name()])
            .add_query_param(
                &FilterFieldName::Filter.name(),
                filter.as_ref().map(|f| f.filter.as_ref()),
            )
            .to_string()?;

        let (identities, total_size) = self
            .api_client
            .send_request_paginated::<_, Identity>(Method::GET, &url, None::<&()>, limit)
            .await?;

        Ok(Identities {
            identities,
            total_size,
        })
    }

    #[named]
    async fn list_groups(
        &self,
        identity_id: &str,
        limit: Option<usize>,
    ) -> Result<Groups, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .add_custom_method(&function_name!().to_case(Case::Camel))
            .to_string()?;

        let (groups, total_size) = self
            .api_client
            .send_request_paginated::<_, Group>(Method::GET, &url, None::<&()>, limit)
            .await?;

        Ok(Groups { groups, total_size })
    }

    #[named]
    async fn list_roles(
        &self,
        identity_id: &str,
        resource_server_id: &str,
        limit: Option<usize>,
    ) -> Result<Roles, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .add_custom_method(&function_name!().to_case(Case::Camel))
            .add_query_param(
                &RoleFieldName::ResourceServerId.name(),
                Some(resource_server_id),
            )
            .to_string()?;

        let (roles, total_size) = self
            .api_client
            .send_request_paginated::<_, Role>(Method::GET, &url, None::<&()>, limit)
            .await?;

        Ok(Roles { roles, total_size })
    }

    async fn patch_identity(&self, request: &PatchIdentityRequest) -> Result<Identity, BiError> {
        self.api_client
            .send_request(
                Method::PATCH,
                &self
                    .api_client
                    .build_url()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![
                        IdentitiesFieldName::Identities.name(),
                        &request.identity.id,
                    ])
                    .to_string()?,
                Some(request),
            )
            .await
    }
}
