use super::api::{IdentitiesApi, IdentityService};
use super::types::{Identities, IdentitiesFieldName, IdentityDetails, IdentityFilterField};
use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::filter::{Filter, FilterFieldName};
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::{
    beyond_identity::api::common::request::send_request_paginated, common::error::BiError,
};
use clap::Args;
use http::Method;
use std::str::FromStr;

// ===============================
// API Function
// ===============================

pub async fn list_identities(
    service: &IdentityService,
    filter: Option<Filter>,
) -> Result<Identities, BiError> {
    let ApiClient {
        config,
        tenant_config,
        client,
    } = &service.api_client;

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
    pub async fn execute(self, service: &IdentityService) -> Result<Identities, BiError> {
        service
            .list_identities(Filter::new(self.filter, IdentityFilterField::from_str)?)
            .await
    }
}
