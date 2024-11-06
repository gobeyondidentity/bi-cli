use super::api::RealmsApi;
use super::types::CreateRealmRequest;
use super::types::PatchRealmRequest;

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::serialize::output;
use crate::beyond_identity::api::common::service::RealmsService;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use field_types::FieldName;

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
        output(RealmsService::new().build().await.create_realm(self)).await
    }
}

// ====================================
// Realms List
// ====================================

#[derive(Debug, Clone, Args, FieldName)]
pub struct List {
    /// Supports filtering realms based on specific fields. Filters follow the SCIM grammar from RFC-7644 Section 3.4.2.2.
    /// https://datatracker.ietf.org/doc/html/rfc7644#section-3.4.2.2
    ///
    /// Acceptable fields:
    ///
    ///   - `id`: The unique identifier for the realm
    ///
    ///   - `display_name`: The display name of the realm
    ///
    /// Example:
    ///
    ///   ---filter "display_name eq \"Production Realm\" or id eq \"8c449e76b1a826ef\""
    #[clap(long)]
    filter: Option<String>,
    #[clap(long, short = 'n')]
    limit: Option<usize>,
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        output(
            RealmsService::new()
                .build()
                .await
                .list_realms(Filter::new(self.filter.clone())?, self.limit),
        )
        .await
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
        output(RealmsService::new().build().await.get_realm(&self.id)).await
    }
}

// ====================================
// Realms Patch
// ====================================

#[async_trait]
impl Executable for PatchRealmRequest {
    async fn execute(&self) -> Result<(), BiError> {
        output(RealmsService::new().build().await.patch_realm(self)).await
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
        output(RealmsService::new().build().await.delete_realm(&self.id)).await
    }
}
