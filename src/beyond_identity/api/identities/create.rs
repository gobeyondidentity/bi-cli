use super::api::{IdentitiesApi, IdentityService};
use super::types::{Identity, IdentityDetails, Traits};
use crate::common::error::BiError;
use clap::Args;
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
