use super::types::{IdentityFilterField, PatchIdentityDetails, PatchIdentityRequest};
use super::{
    api::{IdentitiesApi, IdentityService},
    types::{CreateIdentityRequest, IdentityDetails, IdentityRequest},
};
use crate::beyond_identity::api::common::filter::Filter;
use crate::{beyond_identity::api::common::command::serialize, common::error::BiError};
use clap::Subcommand;
use std::str::FromStr;

#[derive(Subcommand, Debug, Clone)]
pub enum IdentityCommands {
    Create {
        #[clap(flatten)]
        identity_details: IdentityDetails,
    },
    List {
        #[clap(long)]
        filter: Option<String>,
    },
    Get {
        #[clap(long)]
        id: String,
    },
    Patch {
        #[clap(long)]
        id: String,
        #[clap(flatten)]
        identity_details: PatchIdentityDetails,
    },
    Delete {
        #[clap(long)]
        id: String,
    },
    ListGroups {
        #[clap(long)]
        id: String,
    },
    ListRoles {
        #[clap(long)]
        id: String,
        #[clap(long)]
        resource_server_id: String,
    },
}

impl IdentityCommands {
    pub async fn execute(&self, service: &IdentityService) -> Result<String, BiError> {
        match self {
            IdentityCommands::Create { identity_details } => {
                serialize(service.create_identity(CreateIdentityRequest {
                    identity: IdentityRequest {
                        display_name: identity_details.display_name.clone(),
                        traits: identity_details.traits.clone(),
                    },
                }))
                .await
            }
            IdentityCommands::List { filter } => {
                serialize(
                    service.list_identities(Filter::new(
                        filter.clone(),
                        IdentityFilterField::from_str,
                    )?),
                )
                .await
            }
            IdentityCommands::Get { id } => serialize(service.get_identity(id)).await,
            IdentityCommands::Patch {
                id,
                identity_details,
            } => {
                serialize(service.patch_identity(
                    id,
                    &PatchIdentityRequest {
                        identity: PatchIdentityDetails {
                            display_name: identity_details.display_name.clone(),
                            status: identity_details.status.clone(),
                            traits: identity_details.traits.clone(),
                        },
                    },
                ))
                .await
            }
            IdentityCommands::Delete { id } => serialize(service.delete_identity(id)).await,
            IdentityCommands::ListGroups { id } => serialize(service.list_groups(id)).await,
            IdentityCommands::ListRoles {
                id,
                resource_server_id,
            } => serialize(service.list_roles(id, resource_server_id)).await,
        }
    }
}
