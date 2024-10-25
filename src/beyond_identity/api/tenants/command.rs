use super::api::TenantsApi;
use super::types::{PatchTenant, PatchTenantRequest};

use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::Subcommand;

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

#[async_trait]
impl Executable for TenantCommands {
    async fn execute(&self) -> Result<(), BiError> {
        match self {
            TenantCommands::Get => output(Service::new().get_tenant()).await,
            TenantCommands::Patch { display_name } => {
                output(Service::new().patch_tenant(&PatchTenantRequest {
                    tenant: PatchTenant {
                        display_name: Some(display_name.to_string()),
                    },
                }))
                .await
            }
        }
    }
}
