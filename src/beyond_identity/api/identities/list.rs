use super::types::{Identities, IdentitiesFieldName, IdentityDetails, IdentityFilterField};
use crate::beyond_identity::api::utils::filter::{Filter, FilterFieldName};
use crate::beyond_identity::api::utils::url::URLBuilder;
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
    let url = URLBuilder::build(tenant_config)
        .api()
        .add_tenant()
        .add_realm()
        .add_path(vec![IdentitiesFieldName::Identities.name()])
        .add_query_param(
            &FilterFieldName::Filter.name(),
            filter.as_ref().map(|f| f.filter.as_ref()),
        )
        .to_string()?;

    let identities: Vec<IdentityDetails> = send_request_paginated(
        client,
        config,
        tenant_config,
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
            Filter::new(self.filter, IdentityFilterField::from_str)?,
        )
        .await
    }
}
