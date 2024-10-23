use super::api::{IdentitiesApi, IdentityService};
use super::types::{IdentitiesFieldName, Identity, IdentityDetails, Traits};
use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::request::send_request;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::common::error::BiError;
use clap::Args;
use http::Method;
use serde::Serialize;

// ===============================
// Request Structures
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct CreateIdentityRequest {
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

pub async fn create_identity(
    service: &IdentityService,
    identity_request: &CreateIdentityRequest,
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
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service
            .create_identity(CreateIdentityRequest {
                identity: IdentityRequest {
                    display_name: self.identity_details.display_name,
                    traits: self.identity_details.traits,
                },
            })
            .await
    }
}
