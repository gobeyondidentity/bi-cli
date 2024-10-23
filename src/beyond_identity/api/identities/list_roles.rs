use super::api::{IdentitiesApi, IdentityService};
use super::types::IdentitiesFieldName;
use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::beyond_identity::api::roles::types::{
    RoleDetails, RoleDetailsFieldName, Roles, RolesFieldName,
};
use crate::{
    beyond_identity::api::common::request::send_request_paginated, common::error::BiError,
};
use clap::Args;
use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

// ===============================
// API Function
// ===============================

#[named]
pub async fn list_roles(
    service: &IdentityService,
    identity_id: &str,
    resource_server_id: &str,
) -> Result<Roles, BiError> {
    let ApiClient {
        config,
        tenant_config,
        client,
    } = &service.api_client;

    let url = URLBuilder::build(tenant_config)
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
        client,
        config,
        tenant_config,
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

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct ListRoles {
    /// The ID of the identity to list roles for
    #[clap(long)]
    pub id: String,
    /// The ID of the resource server used to filter roles
    #[clap(long)]
    pub resource_server_id: String,
}

impl ListRoles {
    pub async fn execute(self, service: &IdentityService) -> Result<Roles, BiError> {
        service.list_roles(&self.id, &self.resource_server_id).await
    }
}