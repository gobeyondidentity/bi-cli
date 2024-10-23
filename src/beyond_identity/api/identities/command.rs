use super::types::{Identities, IdentityFilterField, PatchIdentityDetails, PatchIdentityRequest};
use super::{
    api::{IdentitiesApi, IdentityService},
    types::{CreateIdentityRequest, Identity, IdentityDetails, IdentityRequest},
};
use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::groups::types::Groups;
use crate::beyond_identity::api::roles::types::Roles;
use crate::{beyond_identity::api::common::command::execute_and_serialize, common::error::BiError};
use clap::{Args, Subcommand};
use std::str::FromStr;

#[derive(Subcommand)]
pub enum IdentityCommands {
    Create(Create),
    List(List),
    Get(Get),
    Patch(Patch),
    Delete(Delete),
    ListGroups(ListGroups),
    ListRoles(ListRoles),
}

impl IdentityCommands {
    pub async fn execute(&self, service: &IdentityService) -> Result<String, BiError> {
        match self {
            IdentityCommands::Create(cmd) => {
                execute_and_serialize(cmd.clone().execute(service)).await
            }
            IdentityCommands::List(cmd) => {
                execute_and_serialize(cmd.clone().execute(service)).await
            }
            IdentityCommands::Get(cmd) => execute_and_serialize(cmd.clone().execute(service)).await,
            IdentityCommands::Patch(cmd) => {
                execute_and_serialize(cmd.clone().execute(service)).await
            }
            IdentityCommands::Delete(cmd) => {
                execute_and_serialize(cmd.clone().execute(service)).await
            }
            IdentityCommands::ListGroups(cmd) => {
                execute_and_serialize(cmd.clone().execute(service)).await
            }
            IdentityCommands::ListRoles(cmd) => {
                execute_and_serialize(cmd.clone().execute(service)).await
            }
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct Create {
    #[clap(flatten)]
    pub identity_details: IdentityDetails,
}

impl Create {
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service
            .create_identity(CreateIdentityRequest {
                identity: IdentityRequest {
                    display_name: self.identity_details.display_name,
                    traits: self.identity_details.traits,
                },
            })
            .await
    }
}

#[derive(Args, Debug, Clone)]
pub struct Delete {
    /// The ID of the identity to retrieve
    #[clap(long)]
    pub id: String,
}

impl Delete {
    pub async fn execute(self, service: &IdentityService) -> Result<serde_json::Value, BiError> {
        service.delete_identity(&self.id).await
    }
}

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// The ID of the identity to retrieve
    #[clap(long)]
    pub id: String,
}

impl Get {
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service.get_identity(&self.id).await
    }
}

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long)]
    pub filter: Option<String>,
}

impl List {
    pub async fn execute(self, service: &IdentityService) -> Result<Identities, BiError> {
        service
            .list_identities(Filter::new(self.filter, IdentityFilterField::from_str)?)
            .await
    }
}

#[derive(Args, Debug, Clone)]
pub struct ListGroups {
    /// The ID of the identity to list groups for
    #[clap(long)]
    pub id: String,
}

impl ListGroups {
    pub async fn execute(self, service: &IdentityService) -> Result<Groups, BiError> {
        service.list_groups(&self.id).await
    }
}

#[derive(Args, Debug, Clone)]
pub struct ListRoles {
    /// The ID of the identity to list roles for
    #[clap(long)]
    pub id: String,
    /// The ID of the resource server used to filter roles
    #[clap(long)]
    pub resource_server_id: String,
}

impl ListRoles {
    pub async fn execute(self, service: &IdentityService) -> Result<Roles, BiError> {
        service.list_roles(&self.id, &self.resource_server_id).await
    }
}

#[derive(Args, Debug, Clone)]
pub struct Patch {
    /// The ID of the identity to patch
    #[clap(long)]
    pub id: String,

    #[clap(flatten)]
    pub identity_details: PatchIdentityDetails,
}

impl Patch {
    pub async fn execute(self, service: &IdentityService) -> Result<Identity, BiError> {
        service
            .patch_identity(
                &self.id,
                &PatchIdentityRequest {
                    identity: PatchIdentityDetails {
                        display_name: self.identity_details.display_name,
                        status: self.identity_details.status,
                        traits: self.identity_details.traits,
                    },
                },
            )
            .await
    }
}
