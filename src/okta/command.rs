use super::fast_migrate;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::middleware::rate_limit::RespectRateLimitMiddleware;
use crate::common::command::ambassador_impl_Executable;
use crate::common::database::models::OktaConfig;
use crate::common::{command::Executable, error::BiError};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

// ====================================
// Okta Commands
// ====================================

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OktaCommands {
    /// Setup allows you to provision an Okta tenant to be used for subsequent commands.
    Setup(Setup),

    /// Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta.
    FastMigrate(FastMigrate),
}

#[derive(Args)]
pub struct Setup {
    /// Okta domain
    #[clap(long)]
    domain: String,

    /// Okta API key
    #[clap(long)]
    api_key: String,

    /// Flag to allow force reconfiguration
    #[clap(long)]
    force: bool,
}

// ====================================
// Okta Setup
// ====================================

#[async_trait]
impl Executable for Setup {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new().await;
        if let Ok(Some(c)) = api_client.db.get_okta_config().await {
            if !self.force {
                println!("Already configured: {:?}", c);
                return Ok(());
            } else {
                println!("Forcing reconfiguration...");
            }
        }
        let okta_config = OktaConfig {
            domain: self.domain.to_string(),
            api_key: self.api_key.to_string(),
        };
        Ok(api_client.db.set_okta_config(okta_config).await?)
    }
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

        let api_client = ApiClient::new().await;
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
