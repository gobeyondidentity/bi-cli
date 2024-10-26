use super::types::{IdentityFilterField, PatchIdentity, PatchIdentityRequest};
use super::{
    api::IdentitiesApi,
    types::{CreateIdentityRequest, Identity, IdentityRequest},
};

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use std::str::FromStr;

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum IdentityCommands {
    /// Create a new identity
    Create(Create),
    /// List identities
    List(List),
    /// Get an identity
    Get(Get),
    /// Update an identity
    Patch(Patch),
    /// Delete an identity
    Delete(Delete),
    /// List an identity's groups
    ListGroups(ListGroups),
    /// List an identity's roles
    ListRoles(ListRoles),
}

#[derive(Args, Debug, Clone)]
pub struct Create {
    #[clap(flatten)]
    identity_details: Identity,
}

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long)]
    filter: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct Get {
    #[clap(long)]
    id: String,
}

#[derive(Args, Debug, Clone)]
pub struct Patch {
    #[clap(long)]
    id: String,
    #[clap(flatten)]
    identity_details: PatchIdentity,
}

#[derive(Args, Debug, Clone)]
pub struct Delete {
    #[clap(long)]
    id: String,
}

#[derive(Args, Debug, Clone)]
pub struct ListGroups {
    #[clap(long)]
    id: String,
}

#[derive(Args, Debug, Clone)]
pub struct ListRoles {
    #[clap(long)]
    id: String,
    #[clap(long)]
    resource_server_id: String,
}

#[async_trait]
impl Executable for Create {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().create_identity(CreateIdentityRequest {
            identity: IdentityRequest {
                display_name: self.identity_details.display_name.clone(),
                traits: self.identity_details.traits.clone(),
            },
        }))
        .await
    }
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().list_identities(Filter::new(
            self.filter.clone(),
            IdentityFilterField::from_str,
        )?))
        .await
    }
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().get_identity(&self.id)).await
    }
}

#[async_trait]
impl Executable for Patch {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().patch_identity(
            &self.id,
            &PatchIdentityRequest {
                identity: PatchIdentity {
                    display_name: self.identity_details.display_name.clone(),
                    status: self.identity_details.status.clone(),
                    traits: self.identity_details.traits.clone(),
                },
            },
        ))
        .await
    }
}

#[async_trait]
impl Executable for Delete {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().delete_identity(&self.id)).await
    }
}

#[async_trait]
impl Executable for ListGroups {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().list_groups(&self.id)).await
    }
}

#[async_trait]
impl Executable for ListRoles {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().list_roles(&self.id, &self.resource_server_id)).await
    }
}
