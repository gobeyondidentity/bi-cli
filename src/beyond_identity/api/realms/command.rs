use super::api::RealmsApi;
use super::types::CreateRealmRequest;
use super::types::PatchRealmRequest;

use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::Service;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};

// ====================================
// Realms Commands
// ====================================

#[derive(Subcommand, Debug, Clone, ambassador::Delegate)]
#[delegate(Executable)]
pub enum RealmCommands {
    /// Create realm
    Create(CreateRealmRequest),

    /// List realms
    List(List),

    /// Get realm
    Get(Get),

    /// Patch realm
    Patch(PatchRealmRequest),

    /// Delete realm
    Delete(Delete),
}

// ====================================
// Realms Create
// ====================================

#[async_trait]
impl Executable for CreateRealmRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().create_realm(self)).await
    }
}

// ====================================
// Realms List
// ====================================

#[derive(Debug, Clone, Args)]
pub struct List;

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().list_realms()).await
    }
}

// ====================================
// Realms Get
// ====================================

#[derive(Debug, Clone, Args)]
pub struct Get {
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().get_realm(&self.id)).await
    }
}

// ====================================
// Realms Patch
// ====================================

#[async_trait]
impl Executable for PatchRealmRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().patch_realm(self)).await
    }
}

// ====================================
// Realms Delete
// ====================================

#[derive(Args, Debug, Clone)]
pub struct Delete {
    #[clap(long)]
    id: String,
}

#[async_trait]
impl Executable for Delete {
    async fn execute(&self) -> Result<(), BiError> {
        output(Service::new().delete_realm(&self.id)).await
    }
}
