use super::api::GroupsApi;
use super::types::{
    AddMembersRequest, CreateGroupRequest, DeleteMembersRequest, PatchGroupRequest,
};

use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use serde::Serialize;

// ====================================
// Identities Commands
// ====================================

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum GroupCommands {
    /// Create a new group
    Create(CreateGroupRequest),
    /// List groups
    List(List),
    /// Get a group
    Get(Get),
    /// Update a group
    Patch(PatchGroupRequest),
    /// Delete a group
    Delete(Delete),
    /// Add members to a group
    AddMembers(AddMembers),
    /// Delete members from a group
    DeleteMembers(DeleteMembers),
    /// List members for a group
    ListMembers(ListMembers),
    /// List role memberships for a group
    ListRoles(ListRoles),
}

// ====================================
// Groups Create
// ====================================

#[async_trait]
impl Executable for CreateGroupRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.create_group(self)).await
    }
}

// ====================================
// Groups List
// ====================================

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.list_groups(self.limit)).await
    }
}

// ====================================
// Groups Get
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Get {
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.get_group(&self.id)).await
    }
}

// ====================================
// Groups Patch
// ====================================

#[async_trait]
impl Executable for PatchGroupRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.patch_group(self)).await
    }
}

// ====================================
// Groups Delete
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Delete {
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Delete {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.delete_group(&self.id)).await
    }
}

// ====================================
// Groups Add Members
// ====================================

#[derive(Args, Debug, Clone, Serialize)]
pub struct AddMembers {
    #[clap(long)]
    id: String,
    #[clap(flatten)]
    request: AddMembersRequest,
}

#[async_trait]
impl Executable for AddMembers {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.add_members(&self.id, &self.request)).await
    }
}

// ====================================
// Groups Delete Members
// ====================================

#[derive(Args, Debug, Clone)]
pub struct DeleteMembers {
    #[clap(long)]
    id: String,
    #[clap(flatten)]
    request: DeleteMembersRequest,
}

#[async_trait]
impl Executable for DeleteMembers {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.delete_members(&self.id, &self.request)).await
    }
}

// ====================================
// Groups List Members
// ====================================

#[derive(Args, Debug, Clone)]
pub struct ListMembers {
    #[clap(long)]
    id: String,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for ListMembers {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().await.list_members(&self.id, self.limit)).await
    }
}

// ====================================
// Groups List Roles
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
