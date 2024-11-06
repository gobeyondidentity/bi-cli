use super::api::GroupsApi;
use super::types::{
    AddMembersRequest, CreateGroupRequest, DeleteMembersRequest, PatchGroupRequest,
};

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::GroupsService;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use field_types::FieldName;
use serde::Serialize;

// ====================================
// Groups Commands
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
        output(GroupsService::new().build().await.create_group(self)).await
    }
}

// ====================================
// Groups List
// ====================================

#[derive(Args, Debug, Clone, FieldName)]
pub struct List {
    /// Supports filtering groups based on specific fields. Filters follow the SCIM grammar from RFC-7644 Section 3.4.2.2.
    /// https://datatracker.ietf.org/doc/html/rfc7644#section-3.4.2.2
    ///
    /// Acceptable fields:
    ///
    ///   - `id`: The unique identifier for the group
    ///
    ///   - `display_name`: The display name of the group
    ///
    /// Example:
    ///
    ///   ---filter "display_name eq \"Engineering\" and id eq \"8c449e76b1a826ef\""
    #[clap(long)]
    filter: Option<String>,

    /// Limits the number of groups returned
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            GroupsService::new()
                .build()
                .await
                .list_groups(Filter::new(self.filter.clone())?, self.limit),
        )
        .await
    }
}

// ====================================
// Groups Get
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// ID of the Group to retrieve
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(GroupsService::new().build().await.get_group(&self.id)).await
    }
}

// ====================================
// Groups Patch
// ====================================

#[async_trait]
impl Executable for PatchGroupRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(GroupsService::new().build().await.patch_group(self)).await
    }
}

// ====================================
// Groups Delete
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Delete {
    /// ID of the Group to delete
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Delete {
    async fn execute(&self) -> Result<(), BiError> {
        output(GroupsService::new().build().await.delete_group(&self.id)).await
    }
}

// ====================================
// Groups Add Members
// ====================================

#[derive(Args, Debug, Clone, Serialize)]
pub struct AddMembers {
    /// ID of the Group to add members to
    #[clap(long)]
    id: String,
    #[clap(flatten)]
    request: AddMembersRequest,
}

#[async_trait]
impl Executable for AddMembers {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            GroupsService::new()
                .build()
                .await
                .add_members(&self.id, &self.request),
        )
        .await
    }
}

// ====================================
// Groups Delete Members
// ====================================

#[derive(Args, Debug, Clone)]
pub struct DeleteMembers {
    /// ID of the Group to delete members from
    #[clap(long)]
    id: String,

    #[clap(flatten)]
    request: DeleteMembersRequest,
}

#[async_trait]
impl Executable for DeleteMembers {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            GroupsService::new()
                .build()
                .await
                .delete_members(&self.id, &self.request),
        )
        .await
    }
}

// ====================================
// Groups List Members
// ====================================

#[derive(Args, Debug, Clone)]
pub struct ListMembers {
    /// ID of the Group to list members for
    #[clap(long)]
    id: String,

    /// Limits the number of members returned
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for ListMembers {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            GroupsService::new()
                .build()
                .await
                .list_members(&self.id, self.limit),
        )
        .await
    }
}

// ====================================
// Groups List Roles
// ====================================

#[derive(Args, Debug, Clone)]
pub struct ListRoles {
    /// ID of the Group to list role memberships for
    #[clap(long)]
    id: String,

    /// ID of the Resource server used to filter roles
    #[clap(long)]
    resource_server_id: String,

    /// Limits the number of roles returned
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for ListRoles {
    async fn execute(&self) -> Result<(), BiError> {
        output(GroupsService::new().build().await.list_roles(
            &self.id,
            &self.resource_server_id,
            self.limit,
        ))
        .await
    }
}
