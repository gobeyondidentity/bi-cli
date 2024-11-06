use super::command::ListFieldName;
use super::types::{
    AddMembersRequest, CreateGroupRequest, DeleteMembersRequest, PatchGroupRequest,
};

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::service::GroupsService;
use crate::beyond_identity::api::groups::types::{Group, Groups, GroupsFieldName};
use crate::beyond_identity::api::identities::types::Identities;
use crate::beyond_identity::api::identities::types::Identity;
use crate::beyond_identity::api::roles::types::Roles;
use crate::beyond_identity::api::roles::types::{Role, RoleFieldName};
use crate::common::error::BiError;

use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

// ====================================
// Groups API
// ====================================

pub trait GroupsApi {
    async fn create_group(&self, request: &CreateGroupRequest) -> Result<Group, BiError>;
    async fn list_groups(
        &self,
        filter: Option<Filter>,
        limit: Option<usize>,
    ) -> Result<Groups, BiError>;
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
    async fn list_members(
        &self,
        group_id: &str,
        limit: Option<usize>,
    ) -> Result<Identities, BiError>;
    async fn list_roles(
        &self,
        group_id: &str,
        resource_server_id: &str,
        limit: Option<usize>,
    ) -> Result<Roles, BiError>;
}

// ====================================
// Groups API Implementation
// ====================================

impl GroupsApi for GroupsService {
    async fn create_group(&self, request: &CreateGroupRequest) -> Result<Group, BiError> {
        self.api_client
            .send_request(
                Method::POST,
                &self
                    .api_client
                    .builder()
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

    async fn list_groups(
        &self,
        filter: Option<Filter>,
        limit: Option<usize>,
    ) -> Result<Groups, BiError> {
        let url = self
            .api_client
            .builder()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![GroupsFieldName::Groups.name()])
            .add_query_param(
                &ListFieldName::Filter.name(),
                filter.as_ref().map(|f| f.0.as_ref()),
            )
            .to_string()?;

        let (groups, total_size) = self
            .api_client
            .send_request_paginated::<_, Group>(Method::GET, &url, None::<&()>, limit)
            .await?;

        Ok(Groups { groups, total_size })
    }

    async fn get_group(&self, group_id: &str) -> Result<Group, BiError> {
        self.api_client
            .send_request(
                Method::GET,
                &self
                    .api_client
                    .builder()
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
                    .builder()
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
                    .builder()
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
                    .builder()
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
                    .builder()
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
    async fn list_members(
        &self,
        group_id: &str,
        limit: Option<usize>,
    ) -> Result<Identities, BiError> {
        let url = self
            .api_client
            .builder()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![GroupsFieldName::Groups.name(), group_id])
            .add_custom_method(&function_name!().to_case(Case::Camel))
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
    async fn list_roles(
        &self,
        group_id: &str,
        resource_server_id: &str,
        limit: Option<usize>,
    ) -> Result<Roles, BiError> {
        let url = self
            .api_client
            .builder()
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

        let (roles, total_size) = self
            .api_client
            .send_request_paginated::<_, Role>(Method::GET, &url, None::<&()>, limit)
            .await?;

        Ok(Roles { roles, total_size })
    }
}
