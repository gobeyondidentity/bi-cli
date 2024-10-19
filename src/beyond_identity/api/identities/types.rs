// Module imports
use super::create::Create;
use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

// ====================================
// Identity Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    pub identity: IdentityDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityDetails {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub display_name: String,
    pub create_time: String,
    pub update_time: String,
    pub traits: Traits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Traits {
    pub r#type: Type,
    pub username: String,
    pub primary_email_address: Option<String>,
    pub external_id: Option<String>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    #[clap(name = "traits_v0")]
    TraitsV0,
}

// ====================================
// Commands for Managing Identities
// ====================================

#[derive(Subcommand)]
pub enum IdentityCommands {
    Create(Create),
}
