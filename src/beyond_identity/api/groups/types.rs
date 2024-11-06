use clap::Args;
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Group Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct Groups {
    pub groups: Vec<Group>,
    pub total_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupEnvelope {
    pub group: Group,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    #[clap(skip)]
    pub id: String,
    #[clap(skip)]
    pub realm_id: String,
    #[clap(skip)]
    pub tenant_id: String,
    /// (required) The display name of the group.
    #[clap(long)]
    pub display_name: String,
    /// (required) A free-form text field to describe a group.
    #[clap(long)]
    pub description: String,
    #[clap(skip)]
    pub create_time: String,
    #[clap(skip)]
    pub update_time: String,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct CreateGroupRequest {
    #[clap(flatten)]
    pub group: CreateGroup,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct CreateGroup {
    #[clap(long)]
    pub display_name: String,
    #[clap(long)]
    pub description: String,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchGroupRequest {
    #[clap(flatten)]
    pub group: PatchGroup,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchGroup {
    #[clap(long)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub description: Option<String>,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct AddMembersRequest {
    /// A list of identity IDs to add as members to the group
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    pub identity_ids: Vec<String>,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct DeleteMembersRequest {
    /// A list of identity IDs to delete from the group
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    pub identity_ids: Vec<String>,
}
