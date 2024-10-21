use super::types::{IdentitiesFieldName, Identity, PatchIdentityDetails};
use crate::{
    beyond_identity::{
        api::utils::{request::send_request, url::URLBuilder},
        tenant::TenantConfig,
    },
    common::{config::Config, error::BiError},
};
use clap::Args;
use http::Method;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::Serialize;

// ===============================
// Request Structures
// ===============================

#[derive(Clone, Debug, Serialize)]
struct PatchIdentityRequest {
    identity: PatchIdentityDetails,
}

// ===============================
// API Function
// ===============================

async fn patch_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
    patch_request: &PatchIdentityRequest,
) -> Result<Identity, BiError> {
    send_request(
        client,
        config,
        tenant_config,
        Method::PATCH,
        &URLBuilder::build(tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name(), identity_id])
            .to_string()?,
        Some(patch_request),
    )
    .await
    .map(|details| Identity { identity: details })
}

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct Patch {
    /// The ID of the identity to patch
    #[clap(long)]
    pub id: String,

    #[clap(flatten)]
    pub identity_details: PatchIdentityDetails,
}

impl Patch {
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Identity, BiError> {
        let patch_request = PatchIdentityRequest {
            identity: PatchIdentityDetails {
                display_name: self.identity_details.display_name,
                status: self.identity_details.status,
                traits: self.identity_details.traits,
            },
        };

        patch_identity(client, config, tenant_config, &self.id, &patch_request).await
    }
}
