mod bi_api_token;
mod bi_scim;
mod config;
mod error;
mod tenant;

use bi_scim::create_beyond_identity_scim_with_okta_registration;
use clap::{Parser, Subcommand};
use config::Config;
use log::LevelFilter;
use reqwest::Client;
use tenant::load_or_create_tenant;

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
    }
}
