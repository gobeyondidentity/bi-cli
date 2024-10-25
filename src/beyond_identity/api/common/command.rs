use super::api_client::ApiClient;

use crate::beyond_identity::api::common::service::Service;
use crate::beyond_identity::api::identities::command::IdentityCommands;
use crate::beyond_identity::api::tenants::command::TenantCommands;
use crate::beyond_identity::helper::tenant::load_tenant;
use crate::common::error::BiError;
use crate::common::{command::Executable, config::Config};

use async_trait::async_trait;
use clap::Subcommand;
use serde::Serialize;

#[derive(Subcommand)]
pub enum BeyondIdentityApiCommands {
    /// Tenants
    #[clap(subcommand)]
    Tenants(TenantCommands),

    /// Identities
    #[clap(subcommand)]
    Identities(IdentityCommands),
}

#[async_trait]
impl Executable for BeyondIdentityApiCommands {
    async fn execute(&self) -> Result<(), BiError> {
        let config = Config::new();
        let tenant_config = load_tenant(&config).expect(
            "Failed to load tenant. Make sure you create a tenant before running this command.",
        );
        let api_client = ApiClient::new(&config, &tenant_config);
        match self {
            BeyondIdentityApiCommands::Identities(cmd) => {
                let result = cmd
                    .execute(&Service::new(api_client))
                    .await
                    .expect("Failed to execute identity command");
                println!("{}", result);
                Ok(())
            }
            BeyondIdentityApiCommands::Tenants(cmd) => {
                let result = cmd
                    .execute(&Service::new(api_client))
                    .await
                    .expect("Failed to execute tenant command");
                println!("{}", result);
                Ok(())
            }
        }
    }
}

pub async fn serialize<T>(
    fut: impl std::future::Future<Output = Result<T, BiError>>,
) -> Result<String, BiError>
where
    T: Serialize,
{
    match fut.await {
        Ok(res) => {
            let json_value = serde_json::to_value(res).map_err(BiError::from)?;
            serde_json::to_string_pretty(&json_value).map_err(BiError::from)
        }
        Err(BiError::RequestError(status, body)) => {
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(parsed_json) => serde_json::to_string_pretty(&parsed_json).or(Ok(body)),
                Err(_) => Ok(format!("Error (HTTP {}): {}", status, body)),
            }
        }
        Err(e) => Err(e),
    }
}
