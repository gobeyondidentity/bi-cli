use super::fast_migrate;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::middleware::rate_limit::RespectRateLimitMiddleware;
use crate::common::command::ambassador_impl_Executable;
use crate::common::{command::Executable, config::OktaConfig, error::BiError};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

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
    domain: String,
    api_key: String,

    /// Flag to allow force reconfiguration
    #[arg(long)]
    force: bool,
}

#[derive(Args)]
pub struct FastMigrate;

#[async_trait]
impl Executable for Setup {
    async fn execute(&self) -> Result<(), BiError> {
        if let Ok(c) = OktaConfig::load_from_file() {
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
        OktaConfig::save_to_file(&okta_config)?;
        Ok(())
    }
}

#[async_trait]
impl Executable for FastMigrate {
    async fn execute(&self) -> Result<(), BiError> {
        let http_client = Client::new();
        let okta_client = ClientBuilder::new(http_client.clone())
            .with(RespectRateLimitMiddleware)
            .build();

        let api_client = ApiClient::new();
        let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");

        let okta_applications = match fast_migrate::load_okta_applications(&api_client.config).await
        {
            Ok(okta_applications) => okta_applications,
            Err(_) => fast_migrate::fetch_okta_applications(
                &okta_client,
                &api_client.config,
                &okta_config,
            )
            .await
            .expect("Failed to fetch okta applications"),
        };

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
