use clap::Subcommand;

use super::api::TenantsApi;
use super::types::{PatchTenant, PatchTenantRequest};

use crate::beyond_identity::api::common::service::Service;
use crate::{beyond_identity::api::common::command::serialize, common::error::BiError};

#[derive(Subcommand, Debug, Clone)]
pub enum TenantCommands {
    /// Get tenant
    Get,
    /// Update tenant
    Patch {
        #[clap(long)]
        display_name: String,
    },
}

impl TenantCommands {
    pub async fn execute(&self, service: &Service) -> Result<String, BiError> {
        match self {
            TenantCommands::Get => serialize(service.get_tenant()).await,
            TenantCommands::Patch { display_name } => {
                serialize(service.patch_tenant(&PatchTenantRequest {
                    tenant: PatchTenant {
                        display_name: Some(display_name.to_string()),
                    },
                }))
                .await
            }
        }
    }
}
