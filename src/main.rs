mod bi_api_token;
mod bi_enrollment;
mod bi_external_sso;
mod bi_scim;
mod config;
mod error;
mod fast_migrate;
mod okta_identity_provider;
mod okta_registration_attribute;
mod okta_routing_rule;
mod okta_scim;
mod tenant;

use bi_enrollment::{get_all_identities, select_identities, send_enrollment_email};
use bi_external_sso::{create_external_sso, load_external_sso};
use bi_scim::{create_beyond_identity_scim_app, load_beyond_identity_scim_app};
use clap::{Parser, Subcommand};
use config::Config;
use fast_migrate::{
    create_sso_config_and_assign_identities, delete_all_sso_configs, fetch_okta_applications,
    load_okta_applications, select_applications,
};
use log::LevelFilter;
use okta_identity_provider::{create_okta_identity_provider, load_okta_identity_provider};
use okta_registration_attribute::{create_custom_attribute, load_custom_attribute};
use okta_routing_rule::{create_okta_routing_rule, load_okta_routing_rule};
use okta_scim::{create_scim_app_in_okta, load_scim_app_in_okta};
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
    CreateCustomAttributeInOkta,
    CreateIdentityProviderInOkta,
    CreateRoutingRuleInOkta,
    SendEnrollmentEmail,
    FastMigrate,
    DeleteAllSSOConfigsInBeyondIdentity,
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
        // Logging is defaulted to info if none is specified.
        _ => LevelFilter::Info,
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
            let okta_app_response = match load_scim_app_in_okta(&config).await {
                Ok(okta_app_response) => okta_app_response,
                Err(_) => create_scim_app_in_okta(&client, &config)
                    .await
                    .expect("Failed to create SCIM app in Okta"),
            };
            println!(
                "Okta SCIM App: {}",
                serde_json::to_string_pretty(&okta_app_response).unwrap()
            );
            println!(
                "Use the following values to configure API provisioning in your Okta Scim App:\nSCIM 2.0 Base Url: {:?}\nOAuth Bearer Token: {:?}",
                format!("{}/v1/tenants/{}/realms/{}/scim/v2", config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id),
                bi_scim_config.oauth_bearer_token
            );
        }
        Commands::CreateExternalSSOConnectionInBeyondIdentity => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let external_sso = match load_external_sso(&config).await {
                Ok(external_sso) => external_sso,
                Err(_) => create_external_sso(&client, &config, &tenant_config)
                    .await
                    .expect("Failed to create External SSO in Beyond Identity"),
            };
            println!(
                "External SSO: {}",
                serde_json::to_string_pretty(&external_sso).unwrap()
            );
        }
        Commands::CreateCustomAttributeInOkta => {
            let config = Config::from_env();
            let client = Client::new();
            let okta_user_schema = match load_custom_attribute(&config).await {
                Ok(okta_user_schema) => okta_user_schema,
                Err(_) => create_custom_attribute(&client, &config)
                    .await
                    .expect("Failed to create custom attribute in Okta"),
            };
            println!(
                "Okta User Schema: {}",
                serde_json::to_string_pretty(&okta_user_schema).unwrap()
            );
        }
        Commands::CreateIdentityProviderInOkta => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let external_sso = load_external_sso(&config).await.expect(
                "Failed to load external sso. Make sure you create an external sso before running this command.",
            );
            let okta_idp = match load_okta_identity_provider(&config).await {
                Ok(okta_idp) => okta_idp,
                Err(_) => {
                    create_okta_identity_provider(&client, &config, &tenant_config, &external_sso)
                        .await
                        .expect("Failed to create an Okta Identity Provider")
                }
            };
            println!(
                "Okta Identity Provider: {}",
                serde_json::to_string_pretty(&okta_idp).unwrap()
            );
        }
        Commands::CreateRoutingRuleInOkta => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let okta_idp_config =  load_okta_identity_provider(&config).await.expect("Failed to load Okta Identity Provider. Make sure you create an Okta Identity Provider before running this command.");
            let okta_routing_rule = match load_okta_routing_rule(&config).await {
                Ok(okta_routing_rule) => okta_routing_rule,
                Err(_) => {
                    create_okta_routing_rule(&client, &config, &tenant_config, &okta_idp_config)
                        .await
                        .expect("Failed to create Okta Routing Rule")
                }
            };
            println!(
                "Okta Routing Rule: {}",
                serde_json::to_string_pretty(&okta_routing_rule).unwrap()
            );
        }
        Commands::SendEnrollmentEmail => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let identity_response = get_all_identities(&client, &config, &tenant_config)
                .await
                .expect("Failed to fetch identities");
            let selected_identities = select_identities(&identity_response.identities);

            for identity in selected_identities {
                match send_enrollment_email(&client, &config, &tenant_config, &identity).await {
                    Ok(job) => println!(
                        "Enrollment job created for {}: {}",
                        identity.traits.primary_email_address,
                        serde_json::to_string_pretty(&job).unwrap()
                    ),
                    Err(err) => println!(
                        "Failed to create enrollment job for {}: {}",
                        identity.traits.primary_email_address, err
                    ),
                }
            }
        }
        Commands::FastMigrate => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );
            let okta_applications = match load_okta_applications(&config).await {
                Ok(okta_applications) => okta_applications,
                Err(_) => fetch_okta_applications(&client, &config)
                    .await
                    .expect("Failed to fetch okta applications"),
            };

            let selected_applications = select_applications(&okta_applications);
            for app in selected_applications {
                match create_sso_config_and_assign_identities(
                    &client,
                    &config,
                    &tenant_config,
                    &app,
                )
                .await
                {
                    Ok(sso_config) => println!(
                        "SSO config created for {}: {}",
                        app.label,
                        serde_json::to_string_pretty(&sso_config).unwrap()
                    ),
                    Err(err) => println!("Failed to create SSO config for {}: {}", app.label, err),
                }
            }
        }
        Commands::DeleteAllSSOConfigsInBeyondIdentity => {
            let config = Config::from_env();
            let client = Client::new();
            let tenant_config = load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            );

            delete_all_sso_configs(&client, &config, &tenant_config)
                .await
                .expect("Failed to delete all SSO Configs");
        }
    }
}
