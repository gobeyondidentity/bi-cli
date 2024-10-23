use super::fast_migrate;
use super::identity_provider::{create_okta_identity_provider, load_okta_identity_provider};
use super::registration_attribute::{create_custom_attribute, load_custom_attribute};
use super::routing_rule::{create_okta_routing_rule, load_okta_routing_rule};
use super::scim::{create_scim_app_in_okta, load_scim_app_in_okta};
use crate::{
    beyond_identity::tenant::load_tenant,
    common::{
        command::Executable,
        config::{Config, OktaConfig},
        error::BiError,
        http::new_http_client_for_api,
    },
};
use async_trait::async_trait;
use clap::Subcommand;

use crate::beyond_identity::external_sso::load_external_sso;
use crate::beyond_identity::scim::load_beyond_identity_scim_app;

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
                Ok(())
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
                Ok(())
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
                Ok(())
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
                Ok(())
            }
            OktaCommands::FastMigrate => {
                let config = Config::new();
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let client = new_http_client_for_api();
                let tenant_config = load_tenant(&config).await.expect(
                            "Failed to load tenant. Make sure you create a tenant before running this command.",
                        );
                let okta_applications = match fast_migrate::load_okta_applications(&config).await {
                    Ok(okta_applications) => okta_applications,
                    Err(_) => fast_migrate::fetch_okta_applications(&client, &config, &okta_config)
                        .await
                        .expect("Failed to fetch okta applications"),
                };

                let selected_applications = fast_migrate::select_applications(&okta_applications);
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
