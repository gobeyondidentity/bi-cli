use super::api::{IdentitiesApi, IdentityService};
use crate::common::error::BiError;
use clap::Args;

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
    pub async fn execute(self, service: &IdentityService) -> Result<serde_json::Value, BiError> {
        service.delete_identity(&self.id).await
    }
}
