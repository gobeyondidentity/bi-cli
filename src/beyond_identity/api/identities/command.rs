use super::types::{IdentityFilterField, PatchIdentityRequest};
use super::{api::IdentitiesApi, types::CreateIdentityRequest};

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use std::str::FromStr;

// ====================================
// Identities Commands
// ====================================

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum IdentityCommands {
    /// Create a new identity
    Create(CreateIdentityRequest),
    /// List identities
    List(List),
    /// Get an identity
    Get(Get),
    /// Update an identity
    Patch(PatchIdentityRequest),
    /// Delete an identity
    Delete(Delete),
    /// List an identity's groups
    ListGroups(ListGroups),
    /// List an identity's roles
    ListRoles(ListRoles),
}

// ====================================
// Identities Create
// ====================================

#[async_trait]
impl Executable for CreateIdentityRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.create_identity(self)).await
    }
}

// ====================================
// Identities List
// ====================================

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long)]
    filter: Option<String>,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.list_identities(
            Filter::new(self.filter.clone(), IdentityFilterField::from_str)?,
            self.limit,
        ))
        .await
    }
}

// ====================================
// Identities Get
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Get {
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.get_identity(&self.id)).await
    }
}

// ====================================
// Identities Patch
// ====================================

#[async_trait]
impl Executable for PatchIdentityRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.patch_identity(self)).await
    }
}

// ====================================
// Identities Delete
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Delete {
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Delete {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.delete_identity(&self.id)).await
    }
}

// ====================================
// Identities ListGroups
// ====================================

#[derive(Args, Debug, Clone)]
pub struct ListGroups {
    #[clap(long)]
    id: String,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for ListGroups {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.list_groups(&self.id, self.limit)).await
    }
}

// ====================================
// Identities ListRoles
// ====================================

#[derive(Args, Debug, Clone)]
pub struct ListRoles {
    #[clap(long)]
    id: String,
    #[clap(long)]
    resource_server_id: String,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for ListRoles {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            Service::new()
                .await
                .list_roles(&self.id, &self.resource_server_id, self.limit),
        )
        .await
    }
}
