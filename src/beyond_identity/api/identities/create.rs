use super::types::{IdentitiesFieldName, Identity, IdentityDetails, Traits};
use crate::beyond_identity::api::common::request::send_request;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::{
    beyond_identity::tenant::TenantConfig,
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
struct CreateIdentityRequest {
    identity: IdentityRequest,
}

#[derive(Clone, Debug, Serialize)]
struct IdentityRequest {
    display_name: String,
    traits: Traits,
}

// ===============================
// API Function
// ===============================

async fn create_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_request: &CreateIdentityRequest,
) -> Result<Identity, BiError> {
    send_request(
        client,
        config,
        tenant_config,
        Method::POST,
        &URLBuilder::build(tenant_config)
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![IdentitiesFieldName::Identities.name()])
            .to_string()?,
        Some(identity_request),
    )
    .await
    .map(|details| Identity { identity: details })
}

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct Create {
    #[clap(flatten)]
    pub identity_details: IdentityDetails,
}

impl Create {
    pub async fn execute(
        self,
        client: &Client,
        config: &Config,
        tenant_config: &TenantConfig,
    ) -> Result<Identity, BiError> {
        let create_request = CreateIdentityRequest {
            identity: IdentityRequest {
                display_name: self.identity_details.display_name,
                traits: self.identity_details.traits,
            },
        };

        create_identity(client, config, tenant_config, &create_request).await
    }
}
