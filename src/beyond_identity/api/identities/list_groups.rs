use crate::beyond_identity::api::groups::types::{GroupDetails, Groups, GroupsFieldName};
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

async fn list_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<Groups, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}:listGroups",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, identity_id
    );

    let groups: Vec<GroupDetails> = send_request_paginated(
        client,
        config,
        tenant_config,
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

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct ListGroups {
    /// The ID of the identity to list groups for
    #[clap(long)]
    pub id: String,
}

impl ListGroups {
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Groups, BiError> {
        list_groups(client, config, tenant_config, &self.id).await
    }
}
