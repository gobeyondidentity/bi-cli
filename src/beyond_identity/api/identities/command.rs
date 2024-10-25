use super::types::{IdentityFilterField, PatchIdentity, PatchIdentityRequest};
use super::{
    api::IdentitiesApi,
    types::{CreateIdentityRequest, Identity, IdentityRequest},
};

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::beyond_identity::helper::tenant::load_tenant;
use crate::common::command::Executable;
use crate::common::config::Config;
use crate::common::error::BiError;

use async_trait::async_trait;
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

#[async_trait]
impl Executable for IdentityCommands {
    async fn execute(&self) -> Result<(), BiError> {
        let config = Config::new();
        let tenant_config = load_tenant(&config)?;
        let api_client = ApiClient::new(&config, &tenant_config);
        let service = Service::new(api_client);
        match self {
            IdentityCommands::Create { identity_details } => {
                output(service.create_identity(CreateIdentityRequest {
                    identity: IdentityRequest {
                        display_name: identity_details.display_name.clone(),
                        traits: identity_details.traits.clone(),
                    },
                }))
                .await
            }
            IdentityCommands::List { filter } => {
                output(
                    service.list_identities(Filter::new(
                        filter.clone(),
                        IdentityFilterField::from_str,
                    )?),
                )
                .await
            }
            IdentityCommands::Get { id } => output(service.get_identity(id)).await,
            IdentityCommands::Patch {
                id,
                identity_details,
            } => {
                output(service.patch_identity(
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
            IdentityCommands::Delete { id } => output(service.delete_identity(id)).await,
            IdentityCommands::ListGroups { id } => output(service.list_groups(id)).await,
            IdentityCommands::ListRoles {
                id,
                resource_server_id,
            } => output(service.list_roles(id, resource_server_id)).await,
        }
    }
}
