use super::api::TenantsApi;
use super::types::{PatchTenant, PatchTenantRequest};

use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum TenantCommands {
    /// Get tenant
    Get(Get),
    /// Update tenant
    Patch(Patch),
}

#[derive(Debug, Clone, Args)]
pub struct Get;

#[derive(Debug, Clone, Args)]
pub struct Patch {
    #[clap(long)]
    display_name: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().get_tenant()).await
    }
}

#[async_trait]
impl Executable for Patch {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().patch_tenant(&PatchTenantRequest {
            tenant: PatchTenant {
                display_name: Some(self.display_name.to_string()),
            },
        }))
        .await
    }
}