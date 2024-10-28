use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Tenant {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Realm {
    pub id: String,
    pub tenant_id: String,
    pub application_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub open_id_configuration_url: String,
    pub auth_base_url: String,
    pub api_base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct OktaConfig {
    pub domain: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct OneloginConfig {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Token {
    pub access_token: String,
    pub expires_at: i64,
    pub tenant_id: String,
    pub realm_id: String,
    pub application_id: String,
}
