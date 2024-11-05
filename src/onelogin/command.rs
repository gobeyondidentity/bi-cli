use super::fast_migrate;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::common::command::ambassador_impl_Executable;
use crate::{
    beyond_identity::api::common::middleware::rate_limit::RespectRateLimitMiddleware,
    common::{command::Executable, error::BiError},
};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

// ====================================
// Onelogin Commands
// ====================================

/// Commands for facilitating migration from OneLogin to Beyond Identity.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OneloginCommands {
    /// Automatically migrate all OneLogin applications to Beyond Identity SSO and assign users based on existing OneLogin assignments.
    /// Each application tile in Beyond Identity will act as an opaque redirect to Onelogin.
    FastMigrate(FastMigrate),
}

// ====================================
// Onelogin FastMigrate
// ====================================

#[derive(Args)]
pub struct FastMigrate;

#[async_trait]
impl Executable for FastMigrate {
    async fn execute(&self) -> Result<(), BiError> {
        let http_client = Client::new();
        let onelogin_client = ClientBuilder::new(http_client.clone())
            .with(RespectRateLimitMiddleware)
            .build();
        let api_client = ApiClient::new(None, None).await;
        let onelogin_config = api_client.db.get_onelogin_config().await?.expect("Failed to load Onelogin Configuration. Make sure to setup Onelogin before running this command.");

        let onelogin_applications =
            fast_migrate::fetch_onelogin_applications(&onelogin_client, &onelogin_config)
                .await
                .expect("Failed to fetch onelogin applications");

        let selected_applications = fast_migrate::select_applications(&onelogin_applications);
        for app in selected_applications {
            match fast_migrate::create_sso_config_and_assign_identities(&api_client, &app).await {
                Ok(sso_config) => println!(
                    "SSO config created for {}: {}",
                    app.name,
                    serde_json::to_string_pretty(&sso_config).unwrap()
                ),
                Err(err) => {
                    println!("Failed to create SSO config for {}: {}", app.name, err)
                }
            }
        }

        Ok(())
    }
}
