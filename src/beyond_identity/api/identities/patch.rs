use super::{
    api::{IdentitiesApi, IdentityService},
    types::{IdentitiesFieldName, Identity, PatchIdentityDetails},
};
use crate::{
    beyond_identity::api::common::{api_client::ApiClient, request::send_request, url::URLBuilder},
    common::error::BiError,
};
use clap::Args;
use http::Method;
use serde::Serialize;

// ===============================
// Request Structures
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct PatchIdentityRequest {
    identity: PatchIdentityDetails,
}

// ===============================
// API Function
// ===============================

pub async fn patch_identity(
    service: &IdentityService,
    identity_id: &str,
    patch_request: &PatchIdentityRequest,
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
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service
            .patch_identity(
                &self.id,
                &PatchIdentityRequest {
                    identity: PatchIdentityDetails {
                        display_name: self.identity_details.display_name,
                        status: self.identity_details.status,
                        traits: self.identity_details.traits,
                    },
                },
            )
            .await
    }
}
