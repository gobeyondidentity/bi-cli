use clap::Args;
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Credential Structures and Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct Credentials {
    pub credentials: Vec<Credential>,
    pub total_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CredentialEnvelope {
    pub credential: Credential,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct Credential {
    #[clap(skip)]
    pub id: String,
    #[clap(skip)]
    pub realm_id: String,
    #[clap(skip)]
    pub tenant_id: String,
    #[clap(long)]
    pub identity_id: String,
    #[clap(skip)]
    pub state: String,
    #[clap(skip)]
    pub csr_type: String,
    #[clap(skip)]
    pub jwk_json: String,
    #[clap(skip)]
    pub jwk_thumbprint: String,
    #[clap(skip)]
    pub create_time: String,
    #[clap(skip)]
    pub update_time: String,
}
