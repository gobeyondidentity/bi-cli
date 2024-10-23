use super::api::{IdentitiesApi, IdentityService};
use crate::beyond_identity::api::roles::types::Roles;
use crate::common::error::BiError;
use clap::Args;

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct ListRoles {
    /// The ID of the identity to list roles for
    #[clap(long)]
    pub id: String,
    /// The ID of the resource server used to filter roles
    #[clap(long)]
    pub resource_server_id: String,
}

impl ListRoles {
    pub async fn execute(self, service: &IdentityService) -> Result<Roles, BiError> {
        service.list_roles(&self.id, &self.resource_server_id).await
    }
}
