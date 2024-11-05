use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::common::command::ambassador_impl_Executable;
use crate::common::database::models::OktaConfig;
use crate::common::{command::Executable, error::BiError};

use async_trait::async_trait;
use clap::{Args, Subcommand};

// ====================================
// Okta Commands
// ====================================

/// Actions for configuring and managing Okta integration settings.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum OktaConfigCommands {
    /// Configure Okta integration settings.
    Set(Set),

    /// Display current Okta integration settings.
    Get(Get),
}

// ====================================
// Okta Set
// ====================================

#[derive(Args)]
pub struct Set {
    /// Okta domain
    #[clap(long)]
    domain: String,

    /// Okta API key
    #[clap(long)]
    api_key: String,

    /// Flag to allow force reconfiguration
    #[clap(long)]
    force: bool,
}

#[async_trait]
impl Executable for Set {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        if let Ok(Some(c)) = api_client.db.get_okta_config().await {
            if !self.force {
                println!("Already configured: {:?}", c);
                return Ok(());
            } else {
                println!("Forcing reconfiguration...");
            }
        }
        let okta_config = OktaConfig {
            domain: self.domain.to_string(),
            api_key: self.api_key.to_string(),
        };
        Ok(api_client.db.set_okta_config(okta_config).await?)
    }
}

// ====================================
// Okta Get
// ====================================

#[derive(Args)]
pub struct Get;

#[async_trait]
impl Executable for Get {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        if let Ok(Some(c)) = api_client.db.get_okta_config().await {
            println!("{:?}", c);
            return Ok(());
        }
        return Err(BiError::StringError("Okta not yet configured".to_string()));
    }
}
