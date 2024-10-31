use super::fast_migrate;

use crate::beyond_identity::api::common::service::Service;
use crate::common::command::ambassador_impl_Executable;
use crate::common::database::models::OneloginConfig;
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

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OneloginCommands {
    /// Setup allows you to provision a Onelogin tenant to be used for subsequent commands.
    Setup(Setup),

    /// Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin.
    FastMigrate(FastMigrate),
}

// ====================================
// Onelogin Setup
// ====================================

#[derive(Args)]
pub struct Setup {
    /// Onelogin domain
    #[clap(long)]
    domain: String,

    /// Onelogin client id
    #[clap(long)]
    client_id: String,

    /// Onelogin client secret
    #[clap(long)]
    client_secret: String,

    /// Flag to allow force reconfiguration
    #[arg(long)]
    force: bool,
}

#[async_trait]
impl Executable for Setup {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = Service::new().build().await.api_client;
        if let Ok(Some(c)) = api_client.db.get_onelogin_config().await {
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
        Ok(api_client.db.set_onelogin_config(onelogin_config).await?)
    }
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
        let api_client = Service::new().build().await.api_client;
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
