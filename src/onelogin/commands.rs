use crate::{
    beyond_identity::tenant::load_tenant,
    common::{
        command::Executable, config::{Config, OneloginConfig}, error::BiError, http::new_http_client_for_api
    },
};
use async_trait::async_trait;
use clap::Subcommand;

use super::fast_migrate;

#[derive(Subcommand)]
pub enum OneloginCommands {
    /// Setup allows you to provision a Onelogin tenant to be used for subsequent commands.
    Setup {
        domain: String,
        client_id: String,
        client_secret: String,

        /// Flag to allow force reconfiguration
        #[arg(long)]
        force: bool,
    },

    /// Automatically populates Beyond Identities SSO with all of your OneLogin applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in OneLogin. Note that each tile you see in Beyond Identity will be an opaque redirect to OneLogin.
    FastMigrate,
}

#[async_trait]
impl Executable for OneloginCommands {
    async fn execute(&self) -> Result<(), BiError> {
        match self {
            OneloginCommands::Setup {
                domain,
                client_id,
                client_secret,
                force,
            } => {
                if let Ok(c) = OneloginConfig::load_from_file() {
                    if !force {
                        println!("Already configured: {:?}", c);
                        return Ok(());
                    } else {
                        println!("Forcing reconfiguration...");
                    }
                }
                let onelogin_config = OneloginConfig {
                    domain: domain.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                };
                OneloginConfig::save_to_file(&onelogin_config)?;
                Ok(())
            }
            OneloginCommands::FastMigrate => {
                let config = Config::new();
                let onelogin_config = OneloginConfig::new().expect("Failed to load Onelogin Configuration. Make sure to setup Onelogin before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let onelogin_applications =
                    match fast_migrate::load_onelogin_applications(&config).await {
                        Ok(onelogin_applications) => onelogin_applications,
                        Err(_) => fast_migrate::fetch_onelogin_applications(
                            &client,
                            &config,
                            &onelogin_config,
                        )
                        .await
                        .expect("Failed to fetch onelogin applications"),
                    };

                let selected_applications =
                    fast_migrate::select_applications(&onelogin_applications);
                for app in selected_applications {
                    match fast_migrate::create_sso_config_and_assign_identities(
                        &client,
                        &config,
                        &tenant_config,
                        &app,
                    )
                    .await
                    {
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
    }
}
