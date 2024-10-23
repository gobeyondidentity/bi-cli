use super::api::{IdentitiesApi, IdentityService};
use crate::beyond_identity::api::groups::types::Groups;
use crate::common::error::BiError;
use clap::Args;

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct ListGroups {
    /// The ID of the identity to list groups for
    #[clap(long)]
    pub id: String,
}

impl ListGroups {
    pub async fn execute(self, service: &IdentityService) -> Result<Groups, BiError> {
        service.list_groups(&self.id).await
    }
}
