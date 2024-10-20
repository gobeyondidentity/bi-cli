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
    filter: Option<Filter<IdentityFilterField>>,
) -> Result<Identities, BiError> {
    let base_url = format!(
        "{}/v1/tenants/{}/realms/{}/identities",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let url = match filter {
        Some(f) => format!("{}?filter={}", base_url, f.to_string()),
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
    fn parse_filter(&self) -> Result<Option<Filter<IdentityFilterField>>, BiError> {
        Filter::parse_with_field_parser(self.filter.as_deref(), |field_str| {
            IdentityFilterField::from_str(field_str).ok()
        })
    }

    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Identities, BiError> {
        let filter = self.parse_filter()?;
        list_identities(client, config, tenant_config, filter).await
    }
}
