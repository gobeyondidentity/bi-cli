use super::tenant::{delete_tenant_ui, list_tenants_ui, provision_tenant, set_default_tenant_ui};

use crate::{
    beyond_identity::api::common::{
        api_client::ApiClient, middleware::rate_limit::RespectRateLimitMiddleware,
    },
    common::{
        command::{ambassador_impl_Executable, Executable},
        error::BiError,
    },
};

use async_trait::async_trait;
use clap::{Args, Subcommand};

/// Actions for managing tenant configurations.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum Tenants {
    /// Provision an existing tenant using the provided API token.
    Add(Add),

    /// Display a list of all configured tenants.
    List(List),

    /// Configure and view the default tenant/realm.
    #[clap(subcommand)]
    Default(DefaultCommands),

    /// Remove a tenant from the configured list.
    Remove(Remove),
}

/// Actions for managing the default tenant/realm.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum DefaultCommands {
    /// Set the default tenant/realm.
    Set(SetDefault),

    /// Get the default tenant/realm.
    Get(GetDefault),
}

#[derive(Args)]
pub struct Add {
    /// The API token associated with the tenant/realm you would like to add.
    #[clap(long)]
    token: String,
}

#[async_trait]
impl Executable for Add {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
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

#[derive(Args)]
pub struct List;

#[async_trait]
impl Executable for List {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        Ok(list_tenants_ui(&api_client.db).await?)
    }
}

#[derive(Args)]
pub struct SetDefault;

#[async_trait]
impl Executable for SetDefault {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        Ok(set_default_tenant_ui(&api_client.db).await?)
    }
}

#[derive(Args)]
pub struct GetDefault;

#[async_trait]
impl Executable for GetDefault {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        Ok(list_tenants_ui(&api_client.db).await?)
    }
}

#[derive(Args)]
pub struct Remove;

#[async_trait]
impl Executable for Remove {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        Ok(delete_tenant_ui(&api_client.db).await?)
    }
}
