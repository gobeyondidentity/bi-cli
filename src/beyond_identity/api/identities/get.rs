use super::api::{IdentitiesApi, IdentityService};
use super::types::Identity;
use crate::common::error::BiError;
use clap::Args;

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// The ID of the identity to retrieve
    #[clap(long)]
    pub id: String,
}

impl Get {
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service.get_identity(&self.id).await
    }
}
