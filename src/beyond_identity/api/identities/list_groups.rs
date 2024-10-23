use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::beyond_identity::api::groups::types::{GroupDetails, Groups, GroupsFieldName};
use crate::{
    beyond_identity::api::common::request::send_request_paginated, common::error::BiError,
};
use clap::Args;
use convert_case::{Case, Casing};
use function_name::named;
use http::Method;

use super::api::{IdentitiesApi, IdentityService};
use super::types::IdentitiesFieldName;

// ===============================
// API Function
// ===============================

#[named]
pub async fn list_groups(service: &IdentityService, identity_id: &str) -> Result<Groups, BiError> {
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
    pub async fn execute(self, service: &IdentityService) -> Result<Groups, BiError> {
        service.list_groups(&self.id).await
    }
}
