use super::api::CredentialsApi;

use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::CredentialsService;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};

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

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long)]
    identity_id: String,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            CredentialsService::new()
                .build()
                .await
                .list_credentials(&self.identity_id, self.limit),
        )
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
