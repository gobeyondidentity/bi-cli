use crate::{
    beyond_identity::tenant::load_tenant,
    common::{
        command::Executable,
        config::{Config, OktaConfig},
        error::BiError,
    },
};
use async_trait::async_trait;
use clap::ArgGroup;
use clap::Subcommand;

use super::enrollment::{
    get_all_identities, get_send_email_payload, get_unenrolled_identities, select_group,
    select_identities, send_enrollment_email,
};
use super::groups::{delete_group_memberships, fetch_all_groups, get_identities_from_group, Group};
use super::identities::{
    delete_all_identities, delete_identity, delete_norole_identities, delete_unenrolled_identities,
    Identity,
};
use super::resource_servers::fetch_beyond_identity_resource_servers;
use super::roles::delete_role_memberships;
use super::scim::{create_beyond_identity_scim_app, load_beyond_identity_scim_app};
use super::sso_configs::delete_all_sso_configs;
use super::tenant::{delete_tenant_ui, list_tenants_ui, provision_tenant, set_default_tenant_ui};
use super::{
    admin::{create_admin_account, get_identities_without_role},
    api::common::middlewares::rate_limit::RespectRateLimitMiddleware,
};
use super::{
    api::common::api_client::ApiClient,
    external_sso::{create_external_sso, load_external_sso},
};

#[derive(Subcommand)]
pub enum BeyondIdentityHelperCommands {
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

/// Enum representing the actions that can be performed in the Setup command.
#[derive(Subcommand)]
pub enum SetupAction {
    /// Provisions an existing tenant using the given API token.
    ProvisionTenant { token: String },

    /// Lists all provisioned tenants.
    ListTenants,

    /// Update which tenant is the default one.
    SetDefaultTenant,

    /// Delete any provisioned tenants.
    DeleteTenant,
}

#[async_trait]
impl Executable for BeyondIdentityHelperCommands {
    async fn execute(&self) -> Result<(), BiError> {
        let config = Config::new();
        let api_client = ApiClient::new(
            &config,
            &load_tenant(&config).await.expect(
                "Failed to load tenant. Make sure you create a tenant before running this command.",
            ),
        );

        match self {
            BeyondIdentityHelperCommands::CreateAdminAccount { email } => {
                let identity = create_admin_account(
                    &api_client.client,
                    &api_client.tenant_config,
                    email.to_string(),
                )
                .await
                .expect("Failed to create admin account");
                println!("Created identity with id={}", identity.id);
                Ok(())
            }
            BeyondIdentityHelperCommands::Setup(action) => match action {
                SetupAction::ProvisionTenant { token } => {
                    _ = provision_tenant(
                        &RespectRateLimitMiddleware::new_client(),
                        &api_client.config,
                        token,
                    )
                    .await
                    .expect("Failed to provision existing tenant");
                    Ok(())
                }
                SetupAction::ListTenants => {
                    list_tenants_ui(&api_client.config)
                        .await
                        .expect("Failed to list tenants");
                    Ok(())
                }
                SetupAction::SetDefaultTenant => {
                    set_default_tenant_ui(&api_client.config)
                        .await
                        .expect("Failed to set default tenant");
                    Ok(())
                }
                SetupAction::DeleteTenant => {
                    delete_tenant_ui(&api_client.config)
                        .await
                        .expect("Failed to delete tenant");
                    Ok(())
                }
            },
            BeyondIdentityHelperCommands::CreateScimApp {
                okta_registration_sync_attribute,
            } => {
                let okta_config = OktaConfig::new().expect("Failed to load Okta Configuration. Make sure to setup Okta before running this command.");
                let bi_scim_app = match load_beyond_identity_scim_app(&api_client.config).await {
                    Ok(bi_scim_app) => bi_scim_app,
                    Err(_) => create_beyond_identity_scim_app(
                        &api_client.client,
                        &api_client.config,
                        &okta_config,
                        &api_client.tenant_config,
                        okta_registration_sync_attribute.clone(),
                    )
                    .await
                    .expect("Failed to create beyond identity scim app"),
                };
                println!(
                    "Beyond Identity SCIM App: {}",
                    serde_json::to_string_pretty(&bi_scim_app).unwrap()
                );
                Ok(())
            }
            BeyondIdentityHelperCommands::CreateExternalSSOConnection => {
                let external_sso = match load_external_sso(&api_client.config).await {
                    Ok(external_sso) => external_sso,
                    Err(_) => {
                        create_external_sso(&api_client.client, &config, &api_client.tenant_config)
                            .await
                            .expect("Failed to create External SSO in Beyond Identity")
                    }
                };
                println!(
                    "External SSO: {}",
                    serde_json::to_string_pretty(&external_sso).unwrap()
                );
                Ok(())
            }
            BeyondIdentityHelperCommands::SendEnrollmentEmail {
                all,
                unenrolled,
                groups,
            } => {
                let mut identities: Vec<Identity> = Vec::new();

                if *all {
                    identities = get_all_identities(&api_client.client, &api_client.tenant_config)
                        .await
                        .expect("Failed to fetch all identities");
                }

                if *unenrolled {
                    identities =
                        get_unenrolled_identities(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to fetch unenrolled identities");
                }

                if *groups {
                    let groups: Vec<Group> =
                        fetch_all_groups(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to fetch groups");
                    let group = select_group(&groups);
                    identities = get_identities_from_group(
                        &api_client.client,
                        &api_client.tenant_config,
                        &group.id,
                    )
                    .await
                    .expect("Failed to fetch identities from group");
                }

                if identities.len() == 0 {
                    println!("No identities found.");
                    return Ok(());
                }

                let selected_identities = select_identities(&identities);

                let payload = get_send_email_payload(&api_client.client, &api_client.tenant_config)
                    .await
                    .expect("Unable to get email payload");

                for identity in selected_identities {
                    match send_enrollment_email(
                        &api_client.client,
                        &api_client.tenant_config,
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
                Ok(())
            }
            BeyondIdentityHelperCommands::DeleteAllSSOConfigs => {
                delete_all_sso_configs(&api_client.client, &api_client.tenant_config)
                    .await
                    .expect("Failed to delete all SSO Configs");
                Ok(())
            }
            BeyondIdentityHelperCommands::DeleteAllIdentities {
                all,
                norole,
                unenrolled,
                force,
            } => {
                if *force {
                    if *all {
                        delete_all_identities(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to delete all identities");
                    }

                    if *unenrolled {
                        delete_unenrolled_identities(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to delete unenrolled identities");
                    }

                    if *norole {
                        delete_norole_identities(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to delete norole identities");
                    }
                    return Ok(());
                }

                let mut identities = vec![];

                if *all {
                    identities = get_all_identities(&api_client.client, &api_client.tenant_config)
                        .await
                        .expect("Failed to fetch all identities");
                }

                if *unenrolled {
                    identities =
                        get_unenrolled_identities(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to fetch unenrolled identities");
                }

                if *norole {
                    identities =
                        get_identities_without_role(&api_client.client, &api_client.tenant_config)
                            .await
                            .expect("Failed to fetch unenrolled identities");
                }

                if identities.len() == 0 {
                    println!("No identities found.");
                    return Ok(());
                }

                let selected_identities = select_identities(&identities);

                let resource_servers = fetch_beyond_identity_resource_servers(
                    &api_client.client,
                    &api_client.tenant_config,
                )
                .await
                .expect("Failed to fetch resource servers");

                for identity in &selected_identities {
                    delete_group_memberships(
                        &api_client.client,
                        &api_client.tenant_config,
                        &identity.id,
                    )
                    .await
                    .expect("Failed to delete role memberships");
                    for rs in &resource_servers {
                        delete_role_memberships(
                            &api_client.client,
                            &api_client.tenant_config,
                            &identity.id,
                            &rs.id,
                        )
                        .await
                        .expect("Failed to delete role memberships");
                    }
                }

                for identity in &selected_identities {
                    delete_identity(&api_client.client, &api_client.tenant_config, &identity.id)
                        .await
                        .expect("Failed to delete identity");
                    println!("Deleted identity {}", identity.id);
                }
                Ok(())
            }
            BeyondIdentityHelperCommands::ReviewUnenrolled => {
                let unenrolled_identities =
                    get_unenrolled_identities(&api_client.client, &api_client.tenant_config)
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
                Ok(())
            }
        }
    }
}
