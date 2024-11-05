use crate::beyond_identity::api::credentials::command::CredentialCommands;
use crate::beyond_identity::api::groups::command::GroupCommands;
use crate::beyond_identity::api::identities::command::IdentityCommands;
use crate::beyond_identity::api::realms::command::RealmCommands;
use crate::beyond_identity::api::tenants::command::TenantCommands;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use clap::Subcommand;

/// Commands for interacting with various Beyond Identity API resources.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum BeyondIdentityApiCommands {
    /// Tenants
    #[clap(subcommand)]
    Tenants(TenantCommands),

    /// Realms
    #[clap(subcommand)]
    Realms(RealmCommands),

    /// Groups
    #[clap(subcommand)]
    Groups(GroupCommands),

    /// Identities
    #[clap(subcommand)]
    Identities(IdentityCommands),

    /// Credentials
    #[clap(subcommand)]
    Credentials(CredentialCommands),
}
