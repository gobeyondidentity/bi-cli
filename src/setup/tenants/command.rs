use super::tenant::{delete_tenant_ui, list_tenants_ui, provision_tenant, set_default_tenant_ui};

use crate::{
    beyond_identity::api::common::middleware::rate_limit::RespectRateLimitMiddleware,
    common::{
        command::{ambassador_impl_Executable, Executable},
        config::Config,
        error::BiError,
    },
};

use async_trait::async_trait;
use clap::{Args, Subcommand};

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum SetupCommands {
    #[clap(subcommand)]
    Tenants(Tenants),
}

/// Tenant management actions.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum Tenants {
    /// Provisions an existing tenant using the provided API token.
    Provision(Provision),

    /// Display a list of all currently provisioned tenants.
    List(List),

    /// Set a specific teannt as the default.
    SetDefault(SetDefault),

    /// Remove a tenant from the list of provisioned tenants.
    Remove(Remove),
}

#[derive(Args)]
pub struct Provision {
    #[clap(long)]
    token: String,
}

#[derive(Args)]
pub struct List;

#[derive(Args)]
pub struct SetDefault;

#[derive(Args)]
pub struct Remove;

#[async_trait]
impl Executable for Provision {
    async fn execute(&self) -> Result<(), BiError> {
        _ = provision_tenant(
            &RespectRateLimitMiddleware::new_client(),
            &Config::new(),
            &self.token,
        )
        .await
        .expect("Failed to provision existing tenant");
        Ok(())
    }
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        list_tenants_ui(&Config::new()).expect("Failed to list tenants");
        Ok(())
    }
}

#[async_trait]
impl Executable for SetDefault {
    async fn execute(&self) -> Result<(), BiError> {
        set_default_tenant_ui(&Config::new()).expect("Failed to set default tenant");
        Ok(())
    }
}

#[async_trait]
impl Executable for Remove {
    async fn execute(&self) -> Result<(), BiError> {
        delete_tenant_ui(&Config::new()).expect("Failed to delete tenant");
        Ok(())
    }
}
