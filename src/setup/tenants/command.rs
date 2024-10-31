use super::tenant::{delete_tenant_ui, list_tenants_ui, provision_tenant, set_default_tenant_ui};

use crate::{
    beyond_identity::api::common::{
        middleware::rate_limit::RespectRateLimitMiddleware, service::Service,
    },
    common::{
        command::{ambassador_impl_Executable, Executable},
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
        let api_client = Service::new().build().await.api_client;
        _ = provision_tenant(
            &RespectRateLimitMiddleware::new_client(),
            &api_client.db,
            &self.token,
        )
        .await
        .expect("Failed to provision tenant/realm");
        Ok(())
    }
}

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = Service::new().build().await.api_client;
        Ok(list_tenants_ui(&api_client.db).await?)
    }
}

#[async_trait]
impl Executable for SetDefault {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = Service::new().build().await.api_client;
        Ok(set_default_tenant_ui(&api_client.db).await?)
    }
}

#[async_trait]
impl Executable for Remove {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = Service::new().build().await.api_client;
        Ok(delete_tenant_ui(&api_client.db).await?)
    }
}
