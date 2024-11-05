use super::admin::{create_admin_account, get_identities_without_role};
use super::enrollment::{
    get_all_identities, get_send_email_payload, get_unenrolled_identities, select_group,
    select_identities, send_enrollment_email,
};
use super::groups::{delete_group_memberships, get_unenrolled_identities_from_group};
use super::identities::{
    delete_all_identities, delete_norole_identities, delete_unenrolled_identities,
};
use super::resource_servers::fetch_beyond_identity_resource_servers;
use super::roles::delete_role_memberships;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::service::{GroupsService, IdentitiesService};
use crate::beyond_identity::api::groups::api::GroupsApi;
use crate::beyond_identity::api::identities::api::IdentitiesApi;
use crate::beyond_identity::api::identities::types::Identity;
use crate::common::command::ambassador_impl_Executable;
use crate::common::{command::Executable, error::BiError};

use async_trait::async_trait;
use clap::{ArgGroup, Args, Subcommand};

/// Helper commands for managing administrative and user-related actions within Beyond Identity.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum BeyondIdentityHelperCommands {
    /// Creates an administrator account in the account.
    CreateAdminAccount(CreateAdminAccount),

    /// Deletes all identities from a realm in case you want to set them up from scratch.
    /// The identities are unassigned from roles and groups automatically.
    #[command(group = ArgGroup::new("delete_option").required(true).multiple(false))]
    DeleteAllIdentities(DeleteAllIdentities),

    /// Helps you send enrollment emails to one or more (or all) users in Beyond Identity.
    #[command(group = ArgGroup::new("delete_option").required(true).multiple(false))]
    SendEnrollmentEmail(SendEnrollmentEmail),

    /// Get a list of identities who have not enrolled yet (identities without a passkey).
    ReviewUnenrolled(ReviewUnenrolled),
}

#[derive(Args)]
pub struct CreateAdminAccount {
    /// Email address of the admin to be created
    email: String,
}

#[derive(Args)]
pub struct DeleteAllIdentities {
    #[arg(long, group = "delete_option")]
    all: bool,

    #[arg(long, group = "delete_option")]
    norole: bool,

    #[arg(long, group = "delete_option")]
    unenrolled: bool,

    /// Skip validation when deleting identities.
    #[arg(long)]
    force: bool,
}

#[derive(Args)]
pub struct SendEnrollmentEmail {
    #[arg(long, group = "delete_option")]
    all: bool,

    #[arg(long, group = "delete_option")]
    groups: bool,

    #[arg(long, requires = "all", requires = "groups")]
    unenrolled: bool,
}

#[derive(Args)]
pub struct ReviewUnenrolled;

#[async_trait]
impl Executable for CreateAdminAccount {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        let identity = create_admin_account(&api_client, self.email.to_string())
            .await
            .expect("Failed to create admin account");
        println!("Created identity with id={}", identity.id);
        Ok(())
    }
}

#[async_trait]
impl Executable for SendEnrollmentEmail {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        let mut identities: Vec<Identity> = Vec::new();

        if self.all {
            if self.unenrolled {
                identities = get_unenrolled_identities(&api_client)
                    .await
                    .expect("Failed to fetch unenrolled identities");
            } else {
                identities = get_all_identities(&api_client)
                    .await
                    .expect("Failed to fetch all identities");
            }
        }

        if self.groups {
            let groups = GroupsService::new()
                .build()
                .await
                .list_groups(None)
                .await?
                .groups;

            if groups.is_empty() {
                println!("No groups found.");
                return Ok(());
            }

            let group = select_group(&groups);

            if self.unenrolled {
                identities = get_unenrolled_identities_from_group(&api_client, &group.id)
                    .await
                    .expect("Failed to fetch unenrolled identities from group");
            } else {
                identities = GroupsService::new()
                    .build()
                    .await
                    .list_members(&group.id, None)
                    .await?
                    .identities;
            }
        }

        if identities.is_empty() {
            println!("No identities found.");
            return Ok(());
        }

        let selected_identities = select_identities(&identities);

        let payload = get_send_email_payload(&api_client)
            .await
            .expect("Unable to get email payload");

        for identity in selected_identities {
            match send_enrollment_email(&api_client, &identity, payload.clone()).await {
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
}

#[async_trait]
impl Executable for DeleteAllIdentities {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        if self.force {
            if self.all {
                delete_all_identities(&api_client)
                    .await
                    .expect("Failed to delete all identities");
            }

            if self.unenrolled {
                delete_unenrolled_identities(&api_client)
                    .await
                    .expect("Failed to delete unenrolled identities");
            }

            if self.norole {
                delete_norole_identities(&api_client)
                    .await
                    .expect("Failed to delete norole identities");
            }
            return Ok(());
        }

        let mut identities = vec![];

        if self.all {
            identities = get_all_identities(&api_client)
                .await
                .expect("Failed to fetch all identities");
        }

        if self.unenrolled {
            identities = get_unenrolled_identities(&api_client)
                .await
                .expect("Failed to fetch unenrolled identities");
        }

        if self.norole {
            identities = get_identities_without_role(&api_client)
                .await
                .expect("Failed to fetch unenrolled identities");
        }

        if identities.len() == 0 {
            println!("No identities found.");
            return Ok(());
        }

        let selected_identities = select_identities(&identities);

        let resource_servers = fetch_beyond_identity_resource_servers(&api_client)
            .await
            .expect("Failed to fetch resource servers");

        for identity in &selected_identities {
            delete_group_memberships(&identity.id)
                .await
                .expect("Failed to delete role memberships");
            for rs in &resource_servers {
                delete_role_memberships(&api_client, &identity.id, &rs.id)
                    .await
                    .expect("Failed to delete role memberships");
            }
        }

        for identity in &selected_identities {
            IdentitiesService::new()
                .build()
                .await
                .delete_identity(&identity.id)
                .await
                .expect("Failed to delete identity");
            println!("Deleted identity {}", identity.id);
        }
        Ok(())
    }
}

#[async_trait]
impl Executable for ReviewUnenrolled {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        let unenrolled_identities = get_unenrolled_identities(&api_client)
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
