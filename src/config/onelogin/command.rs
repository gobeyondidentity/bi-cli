use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::common::command::ambassador_impl_Executable;
use crate::common::database::models::OneloginConfig;
use crate::common::{command::Executable, error::BiError};

use async_trait::async_trait;
use clap::{Args, Subcommand};

// ====================================
// Onelogin Commands
// ====================================

/// Actions for configuring and managing OneLogin integration settings.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OneloginConfigCommands {
    /// Configure OneLogin integration settings.
    Set(Set),

    /// Display current OneLogin integration settings.
    Get(Get),
}

// ====================================
// Onelogin Set
// ====================================

#[derive(Args)]
pub struct Set {
    /// Onelogin domain
    #[clap(long)]
    domain: String,

    /// Onelogin client id
    #[clap(long)]
    client_id: String,

    /// Onelogin client secret
    #[clap(long)]
    client_secret: String,

    /// Flag to allow force reconfiguration
    #[arg(long)]
    force: bool,
}

#[async_trait]
impl Executable for Set {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        if let Ok(Some(c)) = api_client.db.get_onelogin_config().await {
            if !self.force {
                println!("Already configured: {:?}", c);
                return Ok(());
            } else {
                println!("Forcing reconfiguration...");
            }
        }
        let onelogin_config = OneloginConfig {
            domain: self.domain.to_string(),
            client_id: self.client_id.to_string(),
            client_secret: self.client_secret.to_string(),
        };
        Ok(api_client.db.set_onelogin_config(onelogin_config).await?)
    }
}

// ====================================
// Onelogin Get
// ====================================

#[derive(Args)]
pub struct Get;

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        if let Ok(Some(c)) = api_client.db.get_onelogin_config().await {
            println!("{:?}", c);
            return Ok(());
        }
        return Err(BiError::StringError(
            "Onelogin not yet configured".to_string(),
        ));
    }
}
