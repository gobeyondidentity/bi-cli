use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::request::send_request;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::common::error::BiError;
use clap::Args;
use http::Method;

use super::api::{IdentitiesApi, IdentityService};
use super::types::IdentitiesFieldName;

// ===============================
// API Function
// ===============================

pub async fn delete_identity(
    service: &IdentityService,
    identity_id: &str,
) -> Result<serde_json::Value, BiError> {
    let ApiClient {
        config,
        tenant_config,
        client,
    } = &service.api_client;

    send_request(
        client,
        config,
        tenant_config,
        Method::DELETE,
        &URLBuilder::build(tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .to_string()?,
        None::<&()>,
    )
    .await
}

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct Delete {
    /// The ID of the identity to retrieve
    #[clap(long)]
    pub id: String,
}

impl Delete {
    pub async fn execute(self, service: &IdentityService) -> Result<serde_json::Value, BiError> {
        service.delete_identity(&self.id).await
    }
}
