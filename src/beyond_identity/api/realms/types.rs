use clap::{Args, ValueEnum};
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Realm Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct Realms {
    pub realms: Vec<Realm>,
    pub total_size: usize,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct Realm {
    #[clap(skip)]
    pub id: String,
    #[clap(skip)]
    pub tenant_id: String,
    #[clap(long)]
    pub display_name: String,
    #[clap(long, value_enum)]
    pub classification: Option<Classification>,
    #[clap(skip)]
    pub create_time: String,
    #[clap(skip)]
    pub update_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
pub enum Classification {
    #[serde(rename = "Secure Customer")]
    #[clap(name = "secure_customer")]
    SecureCustomer,
    #[serde(rename = "Secure Workforce")]
    #[clap(name = "secure_workforce")]
    SecureWorkforce,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct CreateRealmRequest {
    #[clap(long)]
    pub classification: Classification,
    #[clap(flatten)]
    pub realm: CreateRealm,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct CreateRealm {
    #[clap(long)]
    pub display_name: String,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchRealmRequest {
    #[clap(flatten)]
    pub realm: PatchRealm,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchRealm {
    #[clap(long)]
    pub id: String,
    /// (optional) The display name of the realm.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub display_name: Option<String>,
}
