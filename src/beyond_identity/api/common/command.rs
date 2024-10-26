use crate::beyond_identity::api::identities::command::IdentityCommands;
use crate::beyond_identity::api::realms::command::RealmCommands;
use crate::beyond_identity::api::tenants::command::TenantCommands;
use crate::common::command::ambassador_impl_Executable;
use crate::common::command::Executable;
use crate::common::error::BiError;

use clap::Subcommand;

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum BeyondIdentityApiCommands {
    /// Tenants
    #[clap(subcommand)]
    Tenants(TenantCommands),

    /// Realms
    #[clap(subcommand)]
    Realms(RealmCommands),

    /// Identities
    #[clap(subcommand)]
    Identities(IdentityCommands),
}
