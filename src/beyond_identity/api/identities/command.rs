use super::types::{IdentityFilterField, PatchIdentity, PatchIdentityRequest};
use super::{
    api::IdentitiesApi,
    types::{CreateIdentityRequest, Identity, IdentityRequest},
};
use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::service::Service;
use crate::{beyond_identity::api::common::command::serialize, common::error::BiError};
use clap::Subcommand;
use std::str::FromStr;

#[derive(Subcommand, Debug, Clone)]
pub enum IdentityCommands {
    /// Create a new identity
    Create {
        #[clap(flatten)]
        identity_details: Identity,
    },
    /// List identities
    List {
        #[clap(long)]
        filter: Option<String>,
    },
    /// Get an identity
    Get {
        #[clap(long)]
        id: String,
    },
    /// Update an identity
    Patch {
        #[clap(long)]
        id: String,
        #[clap(flatten)]
        identity_details: PatchIdentity,
    },
    /// Delete an identity
    Delete {
        #[clap(long)]
        id: String,
    },
    /// List an identity's groups
    ListGroups {
        #[clap(long)]
        id: String,
    },
    /// List an identity's roles
    ListRoles {
        #[clap(long)]
        id: String,
        #[clap(long)]
        resource_server_id: String,
    },
}

impl IdentityCommands {
    pub async fn execute(&self, service: &Service) -> Result<String, BiError> {
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
                        identity: PatchIdentity {
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
