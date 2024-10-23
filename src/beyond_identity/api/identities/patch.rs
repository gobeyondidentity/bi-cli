use super::{
    api::{IdentitiesApi, IdentityService},
    types::{Identity, PatchIdentityDetails},
};
use crate::common::error::BiError;
use clap::Args;
use serde::Serialize;

// ===============================
// Request Structures
// ===============================

#[derive(Clone, Debug, Serialize)]
pub struct PatchIdentityRequest {
    identity: PatchIdentityDetails,
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
