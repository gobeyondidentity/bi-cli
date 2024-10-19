use super::types::{Identity, Traits, Type};
use crate::beyond_identity::api::utils::request::send_request;
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
        &format!(
            "{}/v1/tenants/{}/realms/{}/identities",
            tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
        ),
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
    #[clap(long)]
    pub display_name: String,

    #[clap(long, value_enum)]
    pub r#type: Type,

    #[clap(long)]
    pub username: String,

    #[clap(long)]
    pub primary_email_address: Option<String>,

    #[clap(long)]
    pub external_id: Option<String>,

    #[clap(long)]
    pub family_name: Option<String>,

    #[clap(long)]
    pub given_name: Option<String>,
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
                display_name: self.display_name,
                traits: Traits {
                    r#type: self.r#type,
                    username: self.username,
                    primary_email_address: self.primary_email_address,
                    external_id: self.external_id,
                    family_name: self.family_name,
                    given_name: self.given_name,
                },
            },
        };

        create_identity(client, config, tenant_config, &create_request).await
    }
}
