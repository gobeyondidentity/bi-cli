use clap::Args;
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Role Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct Roles {
    pub roles: Vec<Role>,
    pub total_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleEnvelope {
    pub role: Role,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct Role {
    #[clap(skip)]
    pub id: String,
    /// (required) A unique identifier for a resource server.
    #[clap(long)]
    pub resource_server_id: String,
    #[clap(skip)]
    pub realm_id: String,
    #[clap(skip)]
    pub tenant_id: String,
    /// (required) The display name of the role.
    #[clap(long)]
    pub display_name: String,
    /// (required) A free-form text field to describe a role.
    #[clap(long)]
    pub description: String,
    #[clap(skip)]
    pub create_time: String,
    #[clap(skip)]
    pub update_time: String,
}
