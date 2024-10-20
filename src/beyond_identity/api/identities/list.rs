use super::types::{Identities, IdentityDetails, IdentityFilterField};
use crate::beyond_identity::api::utils::filter::Filter;
use crate::{
    beyond_identity::api::utils::request::send_request_paginated,
    beyond_identity::tenant::TenantConfig,
    common::{config::Config, error::BiError},
};
use clap::Args;
use http::Method;
use reqwest_middleware::ClientWithMiddleware as Client;
use std::str::FromStr;

// ===============================
// API Function
// ===============================

async fn list_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    filter: Option<Filter>,
) -> Result<Identities, BiError> {
    let base_url = format!(
        "{}/v1/tenants/{}/realms/{}/identities",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let url = match filter {
        Some(f) => format!("{}?filter={}", base_url, f.encoded),
        None => base_url,
    };

    let identities: Vec<IdentityDetails> = send_request_paginated(
        client,
        config,
        tenant_config,
        Method::GET,
        &url,
        None::<&()>,
        "identities",
    )
    .await?;

    Ok(Identities {
        identities: identities.clone(),
        total_size: identities.len(),
    })
}

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long)]
    pub filter: Option<String>,
}

impl List {
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Identities, BiError> {
        list_identities(
            client,
            config,
            tenant_config,
            Filter::parse_with_fields(self.filter, IdentityFilterField::from_str)?,
        )
        .await
    }
}
