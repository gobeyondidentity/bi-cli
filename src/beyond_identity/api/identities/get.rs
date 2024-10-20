use super::types::Identity;
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

async fn get_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<Identity, BiError> {
    send_request(
        client,
        config,
        tenant_config,
        Method::GET,
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
    .map(|details| Identity { identity: details })
}

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// The ID of the identity to retrieve
    #[clap(long)]
    pub identity_id: String,
}

impl Get {
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Identity, BiError> {
        get_identity(client, config, tenant_config, &self.identity_id).await
    }
}
