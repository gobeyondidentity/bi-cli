use super::identities;
use crate::{
    beyond_identity::tenant::load_tenant,
    common::{command::Executable, config::Config, error::BiError, http::new_http_client_for_api},
};
use async_trait::async_trait;
use clap::Subcommand;

use super::{common::api_client::ApiClient, identities::api::IdentityService};

#[derive(Subcommand)]
pub enum BeyondIdentityApiCommands {
    /// Direct API calls for identities
    #[clap(subcommand)]
    Identities(identities::command::IdentityCommands),
}

#[async_trait]
impl Executable for BeyondIdentityApiCommands {
    async fn execute(&self) -> Result<(), BiError> {
        let config = Config::new();
        let client = new_http_client_for_api();
        let tenant_config = load_tenant(&config).await.expect(
            "Failed to load tenant. Make sure you create a tenant before running this command.",
        );
        let api_client = ApiClient::new(config, tenant_config, client);
        match self {
            BeyondIdentityApiCommands::Identities(cmd) => {
                let result = cmd
                    .execute(&IdentityService::new(api_client))
                    .await
                    .expect("Failed to execute identity command");
                println!("{}", result);
                Ok(())
            }
        }
    }
}
