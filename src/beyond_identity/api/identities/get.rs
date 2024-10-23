use super::api::{IdentitiesApi, IdentityService};
use super::types::{IdentitiesFieldName, Identity};
use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::request::send_request;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::common::error::BiError;
use clap::Args;
use http::Method;

// ===============================
// API Function
// ===============================

pub async fn get_identity(
    service: &IdentityService,
    identity_id: &str,
) -> Result<Identity, BiError> {
    let ApiClient {
        config,
        tenant_config,
        client,
    } = &service.api_client;

    send_request(
        client,
        config,
        tenant_config,
        Method::GET,
        &URLBuilder::build(tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .to_string()?,
        None::<&()>,
    )
    .await
    .map(|details| Identity { identity: details })
}

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// The ID of the identity to retrieve
    #[clap(long)]
    pub id: String,
}

impl Get {
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service.get_identity(&self.id).await
    }
}
