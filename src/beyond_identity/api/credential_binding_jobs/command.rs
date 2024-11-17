use super::api::CredentialBindingJobsApi;
use super::types::CreateCredentialBindingJobRequest;

use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::CredentialBindingJobsService;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};

// ====================================
// Credential Binding Jobs Commands
// ====================================

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum CredentialBindingJobCommands {
    /// Create a credential binding job
    Create(Create),
    /// List credential binding jobs
    List(List),
    /// Get a credential binding job
    Get(Get),
}

// ====================================
// Credential Binding Jobs Create
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Create {
    /// Identity ID associated with the credential binding job
    #[clap(long)]
    identity_id: String,

    #[clap(flatten)]
    request: CreateCredentialBindingJobRequest,
}

#[async_trait]
impl Executable for Create {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            CredentialBindingJobsService::new()
                .build()
                .await
                .create_credential_binding_job(&self.identity_id, &self.request),
        )
        .await
    }
}

// ====================================
// Credential Binding Jobs List
// ====================================

#[derive(Args, Debug, Clone)]
pub struct List {
    /// Identity ID associated with the credential binding job. Identity ID may be a wildcard (-)
    /// to request all credential binding jobs across all identities within the realm.
    #[clap(long)]
    identity_id: String,

    /// Limits the number of credential binding jobs returned
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            CredentialBindingJobsService::new()
                .build()
                .await
                .list_credential_binding_jobs(&self.identity_id, self.limit),
        )
        .await
    }
}

// ====================================
// Credential Binding Jobs Get
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Get {
    /// ID of the credential binding job to retrieve
    #[clap(long)]
    id: String,

    /// Identity ID associated with the credential binding job
    #[clap(long)]
    identity_id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            CredentialBindingJobsService::new()
                .build()
                .await
                .get_credential_binding_job(&self.id, &self.identity_id),
        )
        .await
    }
}
