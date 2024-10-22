use crate::beyond_identity::api::common::request::send_request;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::{
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

async fn delete_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<serde_json::Value, BiError> {
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
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<serde_json::Value, BiError> {
        delete_identity(client, config, tenant_config, &self.id).await
    }
}
