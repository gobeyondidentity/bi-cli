use crate::beyond_identity::api::roles::types::{RoleDetails, Roles};
use crate::{
    beyond_identity::api::utils::request::send_request_paginated,
    beyond_identity::tenant::TenantConfig,
    common::{config::Config, error::BiError},
};
use clap::Args;
use http::Method;
use reqwest_middleware::ClientWithMiddleware as Client;

// ===============================
// API Function
// ===============================

async fn list_roles(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
    resource_server_id: &str,
) -> Result<Roles, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}:listRoles?resource_server_id={}",
        tenant_config.api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        identity_id,
        resource_server_id
    );

    let roles: Vec<RoleDetails> = send_request_paginated(
        client,
        config,
        tenant_config,
        Method::GET,
        &url,
        None::<&()>,
        "roles",
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
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Roles, BiError> {
        list_roles(
            client,
            config,
            tenant_config,
            &self.id,
            &self.resource_server_id,
        )
        .await
    }
}
