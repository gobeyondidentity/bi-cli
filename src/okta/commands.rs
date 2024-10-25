use super::fast_migrate;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::middleware::rate_limit::RespectRateLimitMiddleware;
use crate::{
    beyond_identity::tenant::load_tenant,
    common::{
        command::Executable,
        config::{Config, OktaConfig},
        error::BiError,
    },
};

use async_trait::async_trait;
use clap::Subcommand;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

#[derive(Subcommand)]
pub enum OktaCommands {
    /// Setup allows you to provision an Okta tenant to be used for subsequent commands.
    Setup {
        domain: String,
        api_key: String,

        /// Flag to allow force reconfiguration
        #[arg(long)]
        force: bool,
    },

    /// Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta.
    FastMigrate,
}

#[async_trait]
impl Executable for OktaCommands {
    async fn execute(&self) -> Result<(), BiError> {
        match self {
            OktaCommands::Setup {
                domain,
                api_key,
                force,
            } => {
                if let Ok(c) = OktaConfig::load_from_file() {
                    if !force {
                        println!("Already configured: {:?}", c);
                        return Ok(());
                    } else {
                        println!("Forcing reconfiguration...");
                    }
                }
                let okta_config = OktaConfig {
                    domain: domain.to_string(),
                    api_key: api_key.to_string(),
                };
                OktaConfig::save_to_file(&okta_config)?;
                Ok(())
            }
            OktaCommands::FastMigrate => {
                let http_client = Client::new();
                let okta_client = ClientBuilder::new(http_client.clone())
                    .with(RespectRateLimitMiddleware)
                    .build();

                let config = Config::new();
                let api_client = ApiClient::new(&config, &load_tenant(&config).expect(
                    "Failed to load tenant. Make sure you create a tenant before running this command.",
                ));
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");

                let okta_applications = match fast_migrate::load_okta_applications(&config).await {
                    Ok(okta_applications) => okta_applications,
                    Err(_) => {
                        fast_migrate::fetch_okta_applications(&okta_client, &config, &okta_config)
                            .await
                            .expect("Failed to fetch okta applications")
                    }
                };

                let selected_applications = fast_migrate::select_applications(&okta_applications);
                for app in selected_applications {
                    match fast_migrate::create_sso_config_and_assign_identities(&api_client, &app)
                        .await
                    {
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
    }
}
