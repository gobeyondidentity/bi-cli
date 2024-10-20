use clap::Args;
use serde::{Deserialize, Serialize};

// ====================================
// Identity Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Groups {
    pub groups: Vec<GroupDetails>,
    pub total_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub group: GroupDetails,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct GroupDetails {
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
