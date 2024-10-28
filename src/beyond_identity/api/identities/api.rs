use super::types::{
    CreateIdentityRequest, Identities, IdentitiesFieldName, Identity, IdentityEnvelope,
    PatchIdentityRequest,
};

use crate::beyond_identity::api::common::filter::{Filter, FilterFieldName};
use crate::beyond_identity::api::common::service::Service;
use crate::beyond_identity::api::groups::types::{Group, Groups, GroupsFieldName};
use crate::beyond_identity::api::roles::types::{Role, RoleFieldName, Roles, RolesFieldName};
use crate::common::error::BiError;

use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

// ====================================
// Identities API
// ====================================

pub trait IdentitiesApi {
    async fn create_identity(
        &self,
        request: &CreateIdentityRequest,
    ) -> Result<IdentityEnvelope, BiError>;
    async fn delete_identity(&self, identity_id: &str) -> Result<serde_json::Value, BiError>;
    async fn get_identity(&self, identity_id: &str) -> Result<IdentityEnvelope, BiError>;
    async fn list_identities(&self, filter: Option<Filter>) -> Result<Identities, BiError>;
    async fn list_groups(&self, identity_id: &str) -> Result<Groups, BiError>;
    async fn list_roles(
        &self,
        identity_id: &str,
        resource_server_id: &str,
    ) -> Result<Roles, BiError>;
    async fn patch_identity(
        &self,
        request: &PatchIdentityRequest,
    ) -> Result<IdentityEnvelope, BiError>;
}

// ====================================
// Identities API Implementation
// ====================================

impl IdentitiesApi for Service {
    async fn create_identity(
        &self,
        request: &CreateIdentityRequest,
    ) -> Result<IdentityEnvelope, BiError> {
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
            .map(|details| IdentityEnvelope { identity: details })
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

    async fn get_identity(&self, identity_id: &str) -> Result<IdentityEnvelope, BiError> {
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
            .map(|details| IdentityEnvelope { identity: details })
    }

    async fn list_identities(&self, filter: Option<Filter>) -> Result<Identities, BiError> {
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

        let identities: Vec<Identity> = self
            .api_client
            .send_request_paginated(
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

        let groups: Vec<Group> = self
            .api_client
            .send_request_paginated(
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

        let roles: Vec<Role> = self
            .api_client
            .send_request_paginated(Method::GET, &url, None::<&()>, RolesFieldName::Roles.name())
            .await?;

        Ok(Roles {
            roles: roles.clone(),
            total_size: roles.len(),
        })
    }

    async fn patch_identity(
        &self,
        request: &PatchIdentityRequest,
    ) -> Result<IdentityEnvelope, BiError> {
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
            .map(|details| IdentityEnvelope { identity: details })
    }
}
