use crate::beyond_identity::api::utils::request::send_request;
use crate::{
    beyond_identity::tenant::TenantConfig,
    common::{config::Config, error::BiError},
};
use clap::Args;
use http::Method;
use reqwest_middleware::ClientWithMiddleware as Client;

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
        &format!(
            "{}/v1/tenants/{}/realms/{}/identities/{}",
            tenant_config.api_base_url,
            tenant_config.tenant_id,
            tenant_config.realm_id,
            identity_id
        ),
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
