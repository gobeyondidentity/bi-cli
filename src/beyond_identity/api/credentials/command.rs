use super::api::CredentialsApi;

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::CredentialsService;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use field_types::FieldName;

// ====================================
// Credentials Commands
// ====================================

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum CredentialCommands {
    /// List credentials
    List(List),
    /// Get a credential
    Get(Get),
    /// Revoke a credential
    Revoke(Revoke),
}

// ====================================
// Credentials List
// ====================================

#[derive(Args, Debug, Clone, FieldName)]
pub struct List {
    #[clap(long)]
    identity_id: String,
    /// Supports filtering credentials based on specific fields. Filters follow the SCIM grammar from RFC-7644 Section 3.4.2.2.
    /// https://datatracker.ietf.org/doc/html/rfc7644#section-3.4.2.2
    ///
    /// Acceptable fields:
    ///
    ///   - `state`: The state of the credential. Possible values are [ACTIVE, REVOKED]
    ///
    ///   - `jwk_thumbprint`: The JWK thumbprint of the credential (base64 URL encoded)
    ///
    /// Example:
    ///
    ///   --filter "state eq \"ACTIVE\" and jwk_thumbprint eq \"8BYAqUrR07T_idW89mXkr6hCEIDX6r92coJiXhDWXOA\""
    #[clap(long)]
    filter: Option<String>,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(CredentialsService::new().build().await.list_credentials(
            &self.identity_id,
            Filter::new(self.filter.clone())?,
            self.limit,
        ))
        .await
    }
}

// ====================================
// Credentials Get
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Get {
    #[clap(long)]
    id: String,
    #[clap(long)]
    identity_id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            CredentialsService::new()
                .build()
                .await
                .get_credential(&self.id, &self.identity_id),
        )
        .await
    }
}

// ====================================
// Credentials Revoke
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Revoke {
    #[clap(long)]
    id: String,
    #[clap(long)]
    identity_id: String,
}

#[async_trait]
impl Executable for Revoke {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            CredentialsService::new()
                .build()
                .await
                .revoke_credential(&self.id, &self.identity_id),
        )
        .await
    }
}
