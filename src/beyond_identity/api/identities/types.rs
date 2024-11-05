use clap::{Args, ValueEnum};
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Identity Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct Identities {
    pub identities: Vec<Identity>,
    pub total_size: usize,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    #[clap(skip)]
    pub id: String,
    #[clap(skip)]
    pub realm_id: String,
    #[clap(skip)]
    pub tenant_id: String,
    /// (required) The display name of the identity.
    #[clap(long)]
    pub display_name: String,
    /// (optional) Indicator for the identity's administrative status.
    #[clap(long, value_enum)]
    pub status: Option<Status>,
    #[clap(skip)]
    pub create_time: String,
    #[clap(skip)]
    pub update_time: String,
    #[clap(flatten)]
    pub traits: Traits,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct Traits {
    /// (required) The version of the identity's traits.
    #[clap(long, value_enum)]
    pub r#type: Type,

    /// (required) The unique username associated with the identity.
    #[clap(long)]
    pub username: String,

    /// (optional) The primary email address associated with the identity.
    #[clap(long)]
    pub primary_email_address: Option<String>,

    /// (optional) An external identifier for the identity.
    #[clap(long)]
    pub external_id: Option<String>,

    /// (optional) The family name (surname) of the identity.
    #[clap(long)]
    pub family_name: Option<String>,

    /// (optional) The given name (first name) of the identity.
    #[clap(long)]
    pub given_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    #[clap(name = "traits_v0")]
    TraitsV0,
}

#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Active,
    Suspended,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct CreateIdentityRequest {
    #[clap(flatten)]
    pub identity: CreateIdentity,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct CreateIdentity {
    #[clap(long)]
    pub display_name: String,
    #[clap(flatten)]
    pub traits: Traits,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchIdentityRequest {
    #[clap(flatten)]
    pub identity: PatchIdentity,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchIdentity {
    #[clap(long)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long, value_enum)]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(flatten)]
    pub traits: Option<PatchTraits>,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PatchTraits {
    #[clap(long, value_enum)]
    r#type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    primary_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    given_name: Option<String>,
}
