mod bi_api_token;
mod bi_scim;
mod config;
mod error;
mod tenant;

use bi_scim::create_beyond_identity_scim_with_okta_registration;
use clap::{Parser, Subcommand};
use config::Config;
use reqwest::Client;
use tenant::load_or_create_tenant;

#[derive(Parser)]
#[clap(name = "Provision SSO Tenant")]
#[clap(about = "A CLI tool for setting up an SSO ready Secure Access Tenant", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateTenant,
    CreateScimAppInBeyondIdentity,
    CreateOAuthBearerTokenForScimAppInBeyondIdentity,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateTenant => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_or_create_tenant(&client, &config).await;
            println!(
                "Tenant: {}",
                serde_json::to_string_pretty(&tenant_config).unwrap()
            );
        }
        Commands::CreateScimAppInBeyondIdentity => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_or_create_tenant(&client, &config).await;
            _ = create_beyond_identity_scim_with_okta_registration(
                &client,
                &config,
                &tenant_config,
            )
            .await;
        }
        Commands::CreateOAuthBearerTokenForScimAppInBeyondIdentity => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_or_create_tenant(&client, &config).await;
            _ = create_beyond_identity_scim_with_okta_registration(
                &client,
                &config,
                &tenant_config,
            )
            .await;
        }
    }
}
