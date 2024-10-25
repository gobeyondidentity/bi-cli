use clap::Args;
use serde::{Deserialize, Serialize};

// ====================================
// Tenant Structures and Types
// ====================================

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct Tenant {
    #[clap(skip)]
    pub id: String,
    #[clap(skip)]
    pub display_name: String,
    #[clap(skip)]
    pub create_time: String,
    #[clap(skip)]
    pub update_time: String,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchTenant {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PatchTenantRequest {
    pub tenant: PatchTenant,
}
