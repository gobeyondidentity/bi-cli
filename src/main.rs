mod beyond_identity;
mod common;
mod okta;
mod onelogin;

use beyond_identity::admin::{create_admin_account, get_identities_without_role};
use beyond_identity::api_token::get_beyond_identity_api_token;
use beyond_identity::enrollment::{
    get_all_identities, get_send_email_payload, get_unenrolled_identities, select_group,
    select_identities, send_enrollment_email,
};
use beyond_identity::external_sso::{create_external_sso, load_external_sso};
use beyond_identity::groups::{
    delete_group_memberships, fetch_all_groups, get_identities_from_group, Group,
};
use beyond_identity::identities::{
    delete_all_identities, delete_identity, delete_norole_identities, delete_unenrolled_identities,
    Identity,
};
use beyond_identity::resource_servers::fetch_beyond_identity_resource_servers;
use beyond_identity::roles::delete_role_memberships;
use beyond_identity::scim::{create_beyond_identity_scim_app, load_beyond_identity_scim_app};
use beyond_identity::sso_configs::delete_all_sso_configs;
use beyond_identity::tenant::{
    delete_tenant_ui, list_tenants_ui, load_tenant, provision_tenant, set_default_tenant_ui,
};

use common::http::new_http_client_for_api;
use okta::fast_migrate::{
    create_sso_config_and_assign_identities, fetch_okta_applications, load_okta_applications,
    select_applications,
};
use okta::identity_provider::{create_okta_identity_provider, load_okta_identity_provider};
use okta::registration_attribute::{create_custom_attribute, load_custom_attribute};
use okta::routing_rule::{create_okta_routing_rule, load_okta_routing_rule};
use okta::scim::{create_scim_app_in_okta, load_scim_app_in_okta};

use clap::{ArgGroup, Parser, Subcommand};
use common::config::{Config, OktaConfig, OneloginConfig};
use log::LevelFilter;

#[derive(Parser)]
#[clap(
    name = "bi-cli",
    about = "A CLI tool for setting up an SSO ready Secure Access Tenant",
    version = env!("CARGO_PKG_VERSION"), // Dynamically pulls the version from Cargo.toml
    long_about = None
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long)]
    log_level: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Commands related to Beyond Identity
    #[clap(subcommand)]
    Api(BeyondIdentityCommands),

    /// Commands related to Okta
    #[clap(subcommand)]
    Okta(OktaCommands),

    /// Commands related to OneLogin
    #[clap(subcommand)]
    Onelogin(OneloginCommands),
}

/// Enum representing the actions that can be performed in the Setup command.
#[derive(Subcommand)]
enum SetupAction {
    /// Provisions an existing tenant using the given API token.
    ProvisionTenant { token: String },

    /// Lists all provisioned tenants.
    ListTenants,

    /// Update which tenant is the default one.
    SetDefaultTenant,

    /// Delete any provisioned tenants.
    DeleteTenant,
}

#[derive(Subcommand)]
enum BeyondIdentityCommands {
    /// Provisions configuration for an existing tenant provided an issuer url, client id, and client secret are supplied.
    #[clap(subcommand)]
    Setup(SetupAction),

    /// Creates an application in Beyond Identity that enables you to perform inbound SCIM from an external identity provider.
    CreateScimApp {
        /// Attribute that controls how and when Okta users are routed to Beyond Identity.
        okta_registration_sync_attribute: String,
    },

    /// Creates an OIDC application in Beyond Identity that Okta will use to enable Okta identities to authenticate using Beyond Identity.
    CreateExternalSSOConnection,

    /// Creates an administrator account in the account.
    CreateAdminAccount {
        /// Email address of the admin to be created
        email: String,
    },

    /// Deletes all identities from a realm in case you want to set them up from scratch.
    /// The identities are unassigned from roles and groups automatically.
    #[command(group = ArgGroup::new("delete_option").required(true).multiple(false))]
    DeleteAllIdentities {
        #[arg(long, group = "delete_option")]
        all: bool,

        #[arg(long, group = "delete_option")]
        norole: bool,

        #[arg(long, group = "delete_option")]
        unenrolled: bool,

        /// Skip validation when deleting identities.
        #[arg(long)]
        force: bool,
    },

    /// Get bearer token
    GetToken,

    /// Helps you send enrollment emails to one or more (or all) users in Beyond Identity.
    #[command(group = ArgGroup::new("delete_option").required(true).multiple(false))]
    SendEnrollmentEmail {
        #[arg(long, group = "delete_option")]
        all: bool,

        #[arg(long, group = "delete_option")]
        unenrolled: bool,

        #[arg(long, group = "delete_option")]
        groups: bool,
    },

    /// Clears out your Beyond Identity SSO apps in case you want to run fast migrate from scratch.
    DeleteAllSSOConfigs,

    /// Get a list of identities who have not enrolled yet (identities without a passkey).
    ReviewUnenrolled,
}

#[derive(Subcommand)]
enum OktaCommands {
    /// Setup allows you to provision an Okta tenant to be used for subsequent commands.
    Setup {
        domain: String,
        api_key: String,

        /// Flag to allow force reconfiguration
        #[arg(long)]
        force: bool,
    },

    /// Creates a SCIM app in Okta that is connected to the SCIM app created in the previous step. Note that this command will generate the app and assign all groups to the SCIM app. However, there is a manual step you have to complete on your own which unfortunately cannot be automated. When you run this command the first time, we'll provide you with a SCIM base URL and API token that you'll need to copy into the SCIM app in Okta. You will also have to enable provisioning of identities manually in Okta. The good news is that both of these steps are very easy to do.
    CreateScimApp,

    /// Creates a custom attribute in Okta on the default user type that will be used to create an IDP routing rule in Okta. This is a boolean value that gets set to "true" whenever a passkey is bound for a specific user.
    CreateCustomAttribute {
        /// Attribute that controls how and when Okta users are routed to Beyond Identity.
        okta_registration_sync_attribute: String,
    },

    /// Takes the external SSO connection you created in Beyond Identity and uses it to configure an identity provider in Okta. This is the identity provider that will be used to authenticate Okta users using Beyond Identity.
    CreateIdentityProvider,

    /// The final step when setting up Beyond Identity as an MFA in Okta. This will use the custom attribute you created using an earlier command to route users who have provisioned a Beyond Identity passkey to Beyond Identity during authentication.
    CreateRoutingRule {
        /// Attribute that controls how and when Okta users are routed to Beyond Identity.
        okta_registration_sync_attribute: String,
    },

    /// Automatically populates Beyond Identities SSO with all of your Okta applications. Additionally, it will automatically assign all of your Beyond Identity users to the correct application based on assignments in Okta. Note that each tile you see in Beyond Identity will be an opaque redirect to Okta.
    FastMigrate,
}

#[derive(Subcommand)]
enum OneloginCommands {
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

#[tokio::main]
async fn main() {
    println!("\x1b[31mWARNING: This tool is in alpha. Expect breaking changes.\x1b[0m\n");

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
        Commands::Api(cmd) => match cmd {
            BeyondIdentityCommands::CreateAdminAccount { email } => {
                let config = Config::new();
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let identity =
                    create_admin_account(&client, &config, &tenant_config, email.to_string())
                        .await
                        .expect("Failed to create admin account");
                println!("Created identity with id={}", identity.id);
            }
            BeyondIdentityCommands::Setup(action) => match action {
                SetupAction::ProvisionTenant { token } => {
                    let config = Config::new();
                    let client = new_http_client_for_api();
                    _ = provision_tenant(&client, &config, token)
                        .await
                        .expect("Failed to provision existing tenant");
                }
                SetupAction::ListTenants => {
                    let config = Config::new();
                    list_tenants_ui(&config)
                        .await
                        .expect("Failed to list tenants");
                }
                SetupAction::SetDefaultTenant => {
                    let config = Config::new();
                    set_default_tenant_ui(&config)
                        .await
                        .expect("Failed to set default tenant");
                }
                SetupAction::DeleteTenant => {
                    let config = Config::new();
                    delete_tenant_ui(&config)
                        .await
                        .expect("Failed to delete tenant");
                }
            },
            BeyondIdentityCommands::CreateScimApp {
                okta_registration_sync_attribute,
            } => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let bi_scim_app = match load_beyond_identity_scim_app(&config).await {
                    Ok(bi_scim_app) => bi_scim_app,
                    Err(_) => create_beyond_identity_scim_app(
                        &client,
                        &config,
                        &okta_config,
                        &tenant_config,
                        okta_registration_sync_attribute.clone(),
                    )
                    .await
                    .expect("Failed to create beyond identity scim app"),
                };
                println!(
                    "Beyond Identity SCIM App: {}",
                    serde_json::to_string_pretty(&bi_scim_app).unwrap()
                );
            }
            BeyondIdentityCommands::CreateExternalSSOConnection => {
                let config = Config::new();
                let client = new_http_client_for_api();
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
            BeyondIdentityCommands::SendEnrollmentEmail {
                all,
                unenrolled,
                groups,
            } => {
                let config = Config::new();
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );

                let mut identities: Vec<Identity> = Vec::new();

                if *all {
                    identities = get_all_identities(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch all identities");
                }

                if *unenrolled {
                    identities = get_unenrolled_identities(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch unenrolled identities");
                }

                if *groups {
                    let groups: Vec<Group> = fetch_all_groups(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch groups");
                    let group = select_group(&groups);
                    identities =
                        get_identities_from_group(&client, &config, &tenant_config, &group.id)
                            .await
                            .expect("Failed to fetch identities from group");
                }

                if identities.len() == 0 {
                    println!("No identities found.");
                    return;
                }

                let selected_identities = select_identities(&identities);

                let payload = get_send_email_payload(&client, &config, &tenant_config)
                    .await
                    .expect("Unable to get email payload");

                for identity in selected_identities {
                    match send_enrollment_email(
                        &client,
                        &config,
                        &tenant_config,
                        &identity,
                        payload.clone(),
                    )
                    .await
                    {
                        Ok(job) => println!(
                            "Enrollment job created for {}: {}",
                            identity
                                .traits
                                .primary_email_address
                                .unwrap_or_else(|| "<no email provided>".to_string()),
                            serde_json::to_string_pretty(&job).unwrap()
                        ),
                        Err(err) => println!(
                            "Failed to create enrollment job for {}: {}",
                            identity
                                .traits
                                .primary_email_address
                                .unwrap_or_else(|| "<no email provided>".to_string()),
                            err
                        ),
                    }
                }
            }
            BeyondIdentityCommands::DeleteAllSSOConfigs => {
                let config = Config::new();
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );

                delete_all_sso_configs(&client, &config, &tenant_config)
                    .await
                    .expect("Failed to delete all SSO Configs");
            }
            BeyondIdentityCommands::DeleteAllIdentities {
                all,
                norole,
                unenrolled,
                force,
            } => {
                let config = Config::new();
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );

                if *force {
                    if *all {
                        delete_all_identities(&client, &config, &tenant_config)
                            .await
                            .expect("Failed to delete all identities");
                    }

                    if *unenrolled {
                        delete_unenrolled_identities(&client, &config, &tenant_config)
                            .await
                            .expect("Failed to delete unenrolled identities");
                    }

                    if *norole {
                        delete_norole_identities(&client, &config, &tenant_config)
                            .await
                            .expect("Failed to delete norole identities");
                    }
                    return;
                }

                let mut identities = vec![];

                if *all {
                    identities = get_all_identities(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch all identities");
                }

                if *unenrolled {
                    identities = get_unenrolled_identities(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch unenrolled identities");
                }

                if *norole {
                    identities = get_identities_without_role(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch unenrolled identities");
                }

                if identities.len() == 0 {
                    println!("No identities found.");
                    return;
                }

                let selected_identities = select_identities(&identities);

                let resource_servers =
                    fetch_beyond_identity_resource_servers(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch resource servers");

                for identity in &selected_identities {
                    delete_group_memberships(&client, &config, &tenant_config, &identity.id)
                        .await
                        .expect("Failed to delete role memberships");
                    for rs in &resource_servers {
                        delete_role_memberships(
                            &client,
                            &config,
                            &tenant_config,
                            &identity.id,
                            &rs.id,
                        )
                        .await
                        .expect("Failed to delete role memberships");
                    }
                }

                for identity in &selected_identities {
                    delete_identity(&client, &config, &tenant_config, &identity.id)
                        .await
                        .expect("Failed to delete identity");
                    println!("Deleted identity {}", identity.id);
                }
            }
            BeyondIdentityCommands::GetToken => {
                let config = Config::new();
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let token = get_beyond_identity_api_token(&client, &config, &tenant_config)
                    .await
                    .expect("missing");
                println!("TOKEN: {}", token);
            }
            BeyondIdentityCommands::ReviewUnenrolled => {
                let config = Config::new();
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let unenrolled_identities =
                    get_unenrolled_identities(&client, &config, &tenant_config)
                        .await
                        .expect("Failed to fetch unenrolled identities");

                println!(
                    "{} identities have not completed enrollment yet:",
                    unenrolled_identities.len()
                );
                for identity in unenrolled_identities.iter() {
                    println!(
                        "{} - {}",
                        identity
                            .traits
                            .primary_email_address
                            .as_deref()
                            .unwrap_or("<no email provided>"),
                        identity.id,
                    );
                }
            }
        },
        Commands::Okta(cmd) => match cmd {
            OktaCommands::Setup {
                domain,
                api_key,
                force,
            } => {
                if let Ok(c) = OktaConfig::load_from_file() {
                    if !force {
                        println!("Already configured: {:?}", c);
                        return;
                    } else {
                        println!("Forcing reconfiguration...");
                    }
                }
                let okta_config = OktaConfig {
                    domain: domain.to_string(),
                    api_key: api_key.to_string(),
                };
                OktaConfig::save_to_file(&okta_config)
                    .expect("Failed to save Okta configuration to file")
            }
            OktaCommands::CreateScimApp => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let bi_scim_config = load_beyond_identity_scim_app(&config)
                            .await
                            .expect("Failed to load Beyond Identity SCIM Application. Make sure you create a BI SCIM Application before running this command.");
                let okta_app_response = match load_scim_app_in_okta(&config).await {
                    Ok(okta_app_response) => okta_app_response,
                    Err(_) => create_scim_app_in_okta(&client, &config, &okta_config)
                        .await
                        .expect("Failed to create SCIM app in Okta"),
                };
                println!(
                    "Okta SCIM App: {}",
                    serde_json::to_string_pretty(&okta_app_response).unwrap()
                );
                println!(
                            "Use the following values to configure API provisioning in your Okta Scim App:\nSCIM 2.0 Base Url: {:?}\nOAuth Bearer Token: {:?}",
                            format!("{}/v1/tenants/{}/realms/{}/scim/v2", tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id),
                            bi_scim_config.oauth_bearer_token
                        );
            }
            OktaCommands::CreateCustomAttribute {
                okta_registration_sync_attribute,
            } => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let okta_user_schema = match load_custom_attribute(&config).await {
                    Ok(okta_user_schema) => okta_user_schema,
                    Err(_) => create_custom_attribute(
                        &client,
                        &config,
                        &okta_config,
                        okta_registration_sync_attribute.clone(),
                    )
                    .await
                    .expect("Failed to create custom attribute in Okta"),
                };
                println!(
                    "Okta User Schema: {}",
                    serde_json::to_string_pretty(&okta_user_schema).unwrap()
                );
            }
            OktaCommands::CreateIdentityProvider => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let external_sso = load_external_sso(&config).await.expect(
                            "Failed to load external sso. Make sure you create an external sso before running this command.",
                        );
                let okta_idp = match load_okta_identity_provider(&config).await {
                    Ok(okta_idp) => okta_idp,
                    Err(_) => create_okta_identity_provider(
                        &client,
                        &config,
                        &okta_config,
                        &tenant_config,
                        &external_sso,
                    )
                    .await
                    .expect("Failed to create an Okta Identity Provider"),
                };
                println!(
                    "Okta Identity Provider: {}",
                    serde_json::to_string_pretty(&okta_idp).unwrap()
                );
            }
            OktaCommands::CreateRoutingRule {
                okta_registration_sync_attribute,
            } => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let okta_idp_config =  load_okta_identity_provider(&config).await.expect("Failed to load Okta Identity Provider. Make sure you create an Okta Identity Provider before running this command.");
                let okta_routing_rule = match load_okta_routing_rule(&config).await {
                    Ok(okta_routing_rule) => okta_routing_rule,
                    Err(_) => create_okta_routing_rule(
                        &client,
                        &config,
                        &okta_config,
                        &tenant_config,
                        &okta_idp_config,
                        okta_registration_sync_attribute.clone(),
                    )
                    .await
                    .expect("Failed to create Okta Routing Rule"),
                };
                println!(
                    "Okta Routing Rule: {}",
                    serde_json::to_string_pretty(&okta_routing_rule).unwrap()
                );
            }
            OktaCommands::FastMigrate => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let okta_applications = match load_okta_applications(&config).await {
                    Ok(okta_applications) => okta_applications,
                    Err(_) => fetch_okta_applications(&client, &config, &okta_config)
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
                        Err(err) => {
                            println!("Failed to create SSO config for {}: {}", app.label, err)
                        }
                    }
                }
            }
        },
        Commands::Onelogin(cmd) => match cmd {
            OneloginCommands::Setup {
                domain,
                client_id,
                client_secret,
                force,
            } => {
                if let Ok(c) = OneloginConfig::load_from_file() {
                    if !force {
                        println!("Already configured: {:?}", c);
                        return;
                    } else {
                        println!("Forcing reconfiguration...");
                    }
                }
                let onelogin_config = OneloginConfig {
                    domain: domain.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                };
                OneloginConfig::save_to_file(&onelogin_config)
                    .expect("Failed to save Onelogin configuration to file")
            }
            OneloginCommands::FastMigrate => {
                let config = Config::new();
                let onelogin_config = OneloginConfig::new().expect("Failed to load Onelogin Configuration. Make sure to setup Onelogin before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let onelogin_applications =
                    match onelogin::fast_migrate::load_onelogin_applications(&config).await {
                        Ok(onelogin_applications) => onelogin_applications,
                        Err(_) => onelogin::fast_migrate::fetch_onelogin_applications(
                            &client,
                            &config,
                            &onelogin_config,
                        )
                        .await
                        .expect("Failed to fetch onelogin applications"),
                    };

                let selected_applications =
                    onelogin::fast_migrate::select_applications(&onelogin_applications);
                for app in selected_applications {
                    match onelogin::fast_migrate::create_sso_config_and_assign_identities(
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
            }
        },
    }
}
