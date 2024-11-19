use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::AuthenticatorConfigsService;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};

use super::api::AuthenticatorConfigsApi;
use super::types::CreateAuthenticatorConfigRequest;
use super::types::PatchAuthenticatorConfigRequest;

// ====================================
// Authenticator Configs Commands
// ====================================

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum AuthenticatorConfigCommands {
    /// Create an authenticator config
    Create(CreateAuthenticatorConfigRequest),
    /// List authenticator configs
    List(List),
    /// Get an authenticator config
    Get(Get),
    /// Update an authenticator config
    Patch(PatchAuthenticatorConfigRequest),
    /// Delete an authenticator config
    Delete(Delete),
}

// ====================================
// AuthenticatorConfigs Create
// ====================================

#[async_trait]
impl Executable for CreateAuthenticatorConfigRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            AuthenticatorConfigsService::new()
                .build()
                .await
                .create_authenticator_config(&self),
        )
        .await
    }
}

// ====================================
// Authenticator Configs List
// ====================================

#[derive(Args, Debug, Clone)]
pub struct List {
    /// Limits the number of credential binding jobs returned
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            AuthenticatorConfigsService::new()
                .build()
                .await
                .list_authenticator_configs(self.limit),
        )
        .await
    }
}

// ====================================
// Authenticator Configs Get
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// ID of the Authenticator Config to retrieve
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            AuthenticatorConfigsService::new()
                .build()
                .await
                .get_authenticator_config(&self.id),
        )
        .await
    }
}

// ====================================
// Authenticator Configs Get
// ====================================

#[async_trait]
impl Executable for PatchAuthenticatorConfigRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            AuthenticatorConfigsService::new()
                .build()
                .await
                .patch_authenticator_config(&self),
        )
        .await
    }
}

// ====================================
// Authenticator Configs Delete
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Delete {
    /// ID of the Authenticator Config to delete
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Delete {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            AuthenticatorConfigsService::new()
                .build()
                .await
                .delete_authenticator_config(&self.id),
        )
        .await
    }
}
