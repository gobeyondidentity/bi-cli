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
