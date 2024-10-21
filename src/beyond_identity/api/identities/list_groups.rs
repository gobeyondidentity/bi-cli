use crate::beyond_identity::api::groups::types::{GroupDetails, Groups, GroupsFieldName};
use crate::beyond_identity::api::utils::url::URLBuilder;
use crate::{
    beyond_identity::api::utils::request::send_request_paginated,
    beyond_identity::tenant::TenantConfig,
    common::{config::Config, error::BiError},
};
use clap::Args;
use http::Method;
use reqwest_middleware::ClientWithMiddleware as Client;

use super::types::IdentitiesFieldName;

// ===============================
// API Function
// ===============================

async fn list_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<Groups, BiError> {
    let url = URLBuilder::build(tenant_config)
        .api()
        .add_tenant()
        .add_realm()
        .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
        .add_custom_method("listGroups")
        .to_string()?;

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
