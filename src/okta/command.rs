use super::fast_migrate;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::middleware::rate_limit::RespectRateLimitMiddleware;
use crate::common::command::ambassador_impl_Executable;
use crate::common::{command::Executable, error::BiError};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

// ====================================
// Okta Commands
// ====================================

/// Commands for facilitating migration from Okta to Beyond Identity.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OktaCommands {
    /// Automatically migrate all Okta applications to Beyond Identity SSO and assign users based on existing Okta assignments.
    /// Each application tile in Beyond Identity will act as an opaque redirect to Okta.
    FastMigrate(FastMigrate),
}

// ====================================
// Okta FastMigrate
// ====================================

#[derive(Args)]
pub struct FastMigrate;

#[async_trait]
impl Executable for FastMigrate {
    async fn execute(&self) -> Result<(), BiError> {
        let http_client = Client::new();
        let okta_client = ClientBuilder::new(http_client.clone())
            .with(RespectRateLimitMiddleware)
            .build();

        let api_client = ApiClient::new(None, None).await;
        let okta_config = api_client.db.get_okta_config().await?.expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");

        let okta_applications = fast_migrate::fetch_okta_applications(&okta_client, &okta_config)
            .await
            .expect("Failed to fetch okta applications");

        let selected_applications = fast_migrate::select_applications(&okta_applications);
        for app in selected_applications {
            match fast_migrate::create_sso_config_and_assign_identities(&api_client, &app).await {
                Ok(sso_config) => println!(
                    "SSO config created for {}: {}",
                    app.label,
                    serde_json::to_string_pretty(&sso_config).unwrap()
                ),
                Err(err) => {
                    println!("Failed to create SSO config for {}: {}", app.label, err)
                }
            }
        }
        Ok(())
    }
}
