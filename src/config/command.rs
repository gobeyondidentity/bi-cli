use super::ai::command::Ai;
use super::okta::command::OktaConfigCommands;
use super::onelogin::command::OneloginConfigCommands;
use super::tenants::command::Tenants;

use crate::common::command::{ambassador_impl_Executable, Executable};
use crate::common::error::BiError;

use clap::Subcommand;

/// Commands for configuring various aspects of the CLI tool, enabling interaction with different services and APIs.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum ConfigCommands {
    /// Configure a tenant using an API token to interact with the Beyond Identity API
    #[clap(subcommand)]
    Tenants(Tenants),

    /// Commands for configuring the AI helper tool
    #[clap(subcommand)]
    Ai(Ai),

    /// Configure Okta settings to enable the CLI tool to interact with Okta APIs
    #[clap(subcommand)]
    Okta(OktaConfigCommands),

    /// Configure Onelogin settings to enable the CLI tool to interact with Onelogin APIs
    #[clap(subcommand)]
    Onelogin(OneloginConfigCommands),
}
