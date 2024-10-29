use super::types::{
    AddMembersRequest, CreateGroupRequest, DeleteMembersRequest, PatchGroupRequest,
};

use crate::beyond_identity::api::common::service::Service;
use crate::beyond_identity::api::groups::types::{Group, Groups, GroupsFieldName};
use crate::beyond_identity::api::identities::types::Identity;
use crate::beyond_identity::api::identities::types::{Identities, IdentitiesFieldName};
use crate::beyond_identity::api::roles::types::{Role, RoleFieldName};
use crate::beyond_identity::api::roles::types::{Roles, RolesFieldName};
use crate::common::error::BiError;

use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

// ====================================
// Groups API
// ====================================

pub trait GroupsApi {
    async fn create_group(&self, request: &CreateGroupRequest) -> Result<Group, BiError>;
    async fn list_groups(&self) -> Result<Groups, BiError>;
    async fn get_group(&self, group_id: &str) -> Result<Group, BiError>;
    async fn patch_group(&self, request: &PatchGroupRequest) -> Result<Group, BiError>;
    async fn delete_group(&self, group_id: &str) -> Result<serde_json::Value, BiError>;
    async fn add_members(
        &self,
        group_id: &str,
        request: &AddMembersRequest,
    ) -> Result<Group, BiError>;
    async fn delete_members(
        &self,
        group_id: &str,
        request: &DeleteMembersRequest,
    ) -> Result<Group, BiError>;
    async fn list_members(&self, group_id: &str) -> Result<Identities, BiError>;
    async fn list_roles(&self, group_id: &str, resource_server_id: &str) -> Result<Roles, BiError>;
}

// ====================================
// Groups API Implementation
// ====================================

impl GroupsApi for Service {
    async fn create_group(&self, request: &CreateGroupRequest) -> Result<Group, BiError> {
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
                    .add_path(vec![GroupsFieldName::Groups.name()])
                    .to_string()?,
                Some(request),
            )
            .await
    }

    async fn list_groups(&self) -> Result<Groups, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![GroupsFieldName::Groups.name()])
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

    async fn get_group(&self, group_id: &str) -> Result<Group, BiError> {
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
                    .add_path(vec![GroupsFieldName::Groups.name(), group_id])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    async fn patch_group(&self, request: &PatchGroupRequest) -> Result<Group, BiError> {
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
                    .add_path(vec![GroupsFieldName::Groups.name(), &request.group.id])
                    .to_string()?,
                Some(request),
            )
            .await
    }

    async fn delete_group(&self, group_id: &str) -> Result<serde_json::Value, BiError> {
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
                    .add_path(vec![GroupsFieldName::Groups.name(), group_id])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    #[named]
    async fn add_members(
        &self,
        group_id: &str,
        request: &AddMembersRequest,
    ) -> Result<Group, BiError> {
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
                    .add_path(vec![GroupsFieldName::Groups.name(), group_id])
                    .add_custom_method(&function_name!().to_case(Case::Camel))
                    .to_string()?,
                Some(request),
            )
            .await
    }

    #[named]
    async fn delete_members(
        &self,
        group_id: &str,
        request: &DeleteMembersRequest,
    ) -> Result<Group, BiError> {
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
                    .add_path(vec![GroupsFieldName::Groups.name(), group_id])
                    .add_custom_method(&function_name!().to_case(Case::Camel))
                    .to_string()?,
                Some(request),
            )
            .await
    }

    #[named]
    async fn list_members(&self, group_id: &str) -> Result<Identities, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![GroupsFieldName::Groups.name(), group_id])
            .add_custom_method(&function_name!().to_case(Case::Camel))
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
    async fn list_roles(&self, group_id: &str, resource_server_id: &str) -> Result<Roles, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![GroupsFieldName::Groups.name(), group_id])
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
}