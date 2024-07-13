mod bi_api_token;
mod bi_external_sso;
mod bi_scim;
mod config;
mod error;
mod okta_identity_provider;
mod okta_registration_attribute;
mod okta_scim;
mod tenant;

use bi_external_sso::create_external_sso;
use bi_scim::{create_beyond_identity_scim_app, load_beyond_identity_scim_app};
use clap::{Parser, Subcommand};
use config::Config;
use log::LevelFilter;
use okta_identity_provider::create_okta_identity_provider;
use okta_registration_attribute::add_custom_attribute_to_okta_user_type;
use okta_scim::create_scim_app_in_okta;
use reqwest::Client;
use tenant::{create_tenant, load_tenant};

#[derive(Parser)]
#[clap(name = "Provision SSO Tenant")]
#[clap(about = "A CLI tool for setting up an SSO ready Secure Access Tenant", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long)]
    log_level: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    CreateTenant,
    CreateScimAppInBeyondIdentity,
    CreateScimAppInOkta,
    CreateExternalSSOConnectionInBeyondIdentity,
    AddRegistrationSyncCustomAttributeInOkta,
    CreateIdentityProviderInOkta,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let log_level = match cli.log_level.as_deref() {
        // Use for logging error events that indicate a failure in the application.
        Some("error") => LevelFilter::Error,
        // Use for logging potentially harmful situations that might need attention.
        Some("warn") => LevelFilter::Warn,
        // Use for logging informational messages that highlight the progress of the application.
        Some("info") => LevelFilter::Info,
        // Use for logging detailed information useful for debugging.
        Some("debug") => LevelFilter::Debug,
        // Use for logging very detailed and fine-grained information, typically for tracing program execution.
        Some("trace") => LevelFilter::Trace,
        // Logging is disabled if no flag is present.
        _ => LevelFilter::Off,
    };
    env_logger::Builder::new().filter(None, log_level).init();

    match &cli.command {
        Commands::CreateTenant => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = match load_tenant(&config).await {
                Ok(tenant_config) => tenant_config,
                Err(_) => create_tenant(&client, &config)
                    .await
                    .expect("Failed to create tenant"),
            };
            println!(
                "Tenant: {}",
                serde_json::to_string_pretty(&tenant_config).unwrap()
            );
        }
        Commands::CreateScimAppInBeyondIdentity => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let bi_scim_app = match load_beyond_identity_scim_app(&config).await {
                Ok(bi_scim_app) => bi_scim_app,
                Err(_) => create_beyond_identity_scim_app(&client, &config, &tenant_config)
                    .await
                    .expect("Failed to create beyond identity scim app"),
            };
            println!(
                "Beyond Identity SCIM App: {}",
                serde_json::to_string_pretty(&bi_scim_app).unwrap()
            );
        }
        Commands::CreateScimAppInOkta => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let bi_scim_config = load_beyond_identity_scim_app(&config)
                .await
                .expect("Failed to load Beyond Identity SCIM Application. Make sure you create a BI SCIM Application before running this command.");
            let okta_app_response = create_scim_app_in_okta(&client, &config).await;
            println!(
                "Okta App: {}",
                serde_json::to_string_pretty(&okta_app_response).unwrap()
            );
            println!("Use the following values to configure API provisioning in your Okta Scim App:\nSCIM 2.0 Base Url: {:?}\nOAuth Bearer Token: {:?}", format!("{}/v1/tenants/{}/realms/{}/scim/v2", config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id), bi_scim_config.oauth_bearer_token);
        }
        Commands::CreateExternalSSOConnectionInBeyondIdentity => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let external_sso = create_external_sso(&client, &config, &tenant_config).await;
            println!(
                "External SSO: {}",
                serde_json::to_string_pretty(&external_sso).unwrap()
            );
        }
        Commands::AddRegistrationSyncCustomAttributeInOkta => {
            let config = Config::from_env();
            let client = Client::new();
            add_custom_attribute_to_okta_user_type(&client, &config)
                .await
                .expect("Failed to create custom attribute in Okta");
        }
        Commands::CreateIdentityProviderInOkta => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let external_sso = create_external_sso(&client, &config, &tenant_config).await;
            create_okta_identity_provider(&client, &config, &tenant_config, &external_sso).await;
        }
    }
}
