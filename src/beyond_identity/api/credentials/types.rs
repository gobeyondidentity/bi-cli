use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Credential Types
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

/// Represents a credential resource in the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credential {
    /// A unique identifier for the credential.
    pub id: String,

    /// A unique identifier for the realm this credential belongs to.
    pub realm_id: String,

    /// A unique identifier for the tenant this credential belongs to.
    pub tenant_id: String,

    /// A unique identifier for the identity associated with this credential.
    pub identity_id: String,

    /// The state of the credential (e.g., ACTIVE, INACTIVE).
    pub state: String,

    /// The type of CSR (Certificate Signing Request) associated with the credential.
    /// Example: "JWT".
    pub csr_type: String,

    /// A JSON representation of the JWK (JSON Web Key) associated with the credential.
    pub jwk_json: String,

    /// A thumbprint of the JWK, used for verifying the key.
    pub jwk_thumbprint: String,

    /// The timestamp when the credential was created represented as an ISO 8601 string.
    pub create_time: String,

    /// The timestamp when the credential was last updated represented as an ISO 8601 string.
    pub update_time: String,
}
