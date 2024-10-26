use super::fast_migrate;

use crate::common::command::ambassador_impl_Executable;
use crate::{
    beyond_identity::api::common::{
        api_client::ApiClient, middleware::rate_limit::RespectRateLimitMiddleware,
    },
    common::{command::Executable, config::OneloginConfig, error::BiError},
};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OneloginCommands {
    /// Setup allows you to provision a Onelogin tenant to be used for subsequent commands.
    Setup(Setup),

    /// Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin.
    FastMigrate(FastMigrate),
}

#[derive(Args)]
pub struct Setup {
    domain: String,
    client_id: String,
    client_secret: String,

    /// Flag to allow force reconfiguration
    #[arg(long)]
    force: bool,
}

#[derive(Args)]
pub struct FastMigrate;

#[async_trait]
impl Executable for Setup {
    async fn execute(&self) -> Result<(), BiError> {
        if let Ok(c) = OneloginConfig::load_from_file() {
            if !self.force {
                println!("Already configured: {:?}", c);
                return Ok(());
            } else {
                println!("Forcing reconfiguration...");
            }
        }
        let onelogin_config = OneloginConfig {
            domain: self.domain.to_string(),
            client_id: self.client_id.to_string(),
            client_secret: self.client_secret.to_string(),
        };
        OneloginConfig::save_to_file(&onelogin_config)?;
        Ok(())
    }
}

#[async_trait]
impl Executable for FastMigrate {
    async fn execute(&self) -> Result<(), BiError> {
        let http_client = Client::new();
        let onelogin_client = ClientBuilder::new(http_client.clone())
            .with(RespectRateLimitMiddleware)
            .build();
        let api_client = ApiClient::new();
        let onelogin_config = OneloginConfig::new().expect("Failed to load Onelogin Configuration. Make sure to setup Onelogin before running this command.");

        let onelogin_applications =
            match fast_migrate::load_onelogin_applications(&api_client.config).await {
                Ok(onelogin_applications) => onelogin_applications,
                Err(_) => fast_migrate::fetch_onelogin_applications(
                    &onelogin_client,
                    &api_client.config,
                    &onelogin_config,
                )
                .await
                .expect("Failed to fetch onelogin applications"),
            };

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
