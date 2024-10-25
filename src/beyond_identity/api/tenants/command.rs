use super::api::TenantsApi;
use super::types::{PatchTenant, PatchTenantRequest};

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::beyond_identity::helper::tenant::load_tenant;
use crate::common::command::Executable;
use crate::common::config::Config;
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
        let config = Config::new();
        let tenant_config = load_tenant(&config)?;
        let api_client = ApiClient::new(&config, &tenant_config);
        let service = Service::new(api_client);
        match self {
            TenantCommands::Get => output(service.get_tenant()).await,
            TenantCommands::Patch { display_name } => {
                output(service.patch_tenant(&PatchTenantRequest {
                    tenant: PatchTenant {
                        display_name: Some(display_name.to_string()),
                    },
                }))
                .await
            }
        }
    }
}
