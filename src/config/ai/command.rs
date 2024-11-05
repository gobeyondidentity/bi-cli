use crate::{
    beyond_identity::api::common::api_client::ApiClient,
    common::{
        command::{ambassador_impl_Executable, Executable},
        database::models::{AiProvider, AnthropicConfig, OpenaiConfig},
        error::BiError,
    },
};

use async_trait::async_trait;
use clap::{Args, Subcommand};

/// Actions for managing AI provider configurations.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum Ai {
    /// Configure settings related to an AI provider.
    #[clap(subcommand)]
    Provider(ProviderCommands),

    /// Configure and view the default AI provider.
    #[clap(subcommand)]
    Default(DefaultCommands),
}

/// Actions for settings and displaying AI provider configuration.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum ProviderCommands {
    /// Set AI provider configuration.
    Set(SetProvider),

    /// Get AI provider configuration.
    Get(GetProvider),
}

/// Actions for setting and displaying the default AI provider.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum DefaultCommands {
    /// Configure the default AI provider.
    Set(SetDefault),

    /// Display the current default AI provider.
    Get(GetDefault),
}

// ====================================
// Provider Set
// ====================================

#[derive(Args)]
pub struct SetProvider {
    /// AI Provider
    #[clap(long, value_enum)]
    pub provider: AiProvider,

    /// API Key
    #[clap(long)]
    pub api_key: String,

    /// Flag to allow force reconfiguration
    #[arg(long)]
    force: bool,
}

#[async_trait]
impl Executable for SetProvider {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;

        match self.provider {
            AiProvider::Openai => {
                if let Ok(Some(c)) = api_client.db.get_openai_config().await {
                    if !self.force {
                        println!("Already configured: {:?}", c);
                        return Ok(());
                    } else {
                        println!("Forcing reconfiguration for OpenAI...");
                    }
                }
                let openai_config = OpenaiConfig {
                    api_key: self.api_key.to_string(),
                };

                Ok(api_client.db.set_openai_config(openai_config).await?)
            }
            AiProvider::Anthropic => {
                if let Ok(Some(c)) = api_client.db.get_anthropic_config().await {
                    if !self.force {
                        println!("Already configured: {:?}", c);
                        return Ok(());
                    } else {
                        println!("Forcing reconfiguration for Anthropic...");
                    }
                }
                let anthropic_config = AnthropicConfig {
                    api_key: self.api_key.to_string(),
                };

                Ok(api_client.db.set_anthropic_config(anthropic_config).await?)
            }
        }
    }
}

// ====================================
// Provider Get
// ====================================

#[derive(Args)]
pub struct GetProvider {
    /// AI Provider
    #[clap(long, value_enum)]
    pub provider: AiProvider,
}

#[async_trait]
impl Executable for GetProvider {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        match self.provider {
            AiProvider::Openai => {
                if let Ok(Some(c)) = api_client.db.get_openai_config().await {
                    println!("{:?}", c);
                    return Ok(());
                }
                Err(BiError::StringError(
                    "OpenAI not yet configured".to_string(),
                ))
            }
            AiProvider::Anthropic => {
                if let Ok(Some(c)) = api_client.db.get_anthropic_config().await {
                    println!("{:?}", c);
                    return Ok(());
                }
                Err(BiError::StringError(
                    "Anthropic not yet configured".to_string(),
                ))
            }
        }
    }
}

// ====================================
// Default Set
// ====================================

#[derive(Args, Clone)]
pub struct SetDefault {
    #[clap(long, value_enum)]
    pub provider: AiProvider,
}

#[async_trait]
impl Executable for SetDefault {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        Ok(api_client
            .db
            .set_default_ai_provider(self.provider.clone())
            .await?)
    }
}

// ====================================
// Default Get
// ====================================

#[derive(Args)]
pub struct GetDefault;

#[async_trait]
impl Executable for GetDefault {
    async fn execute(&self) -> Result<(), BiError> {
        let api_client = ApiClient::new(None, None).await;
        if let Ok(Some(c)) = api_client.db.get_default_ai_provider().await {
            println!("{:?}", c);
            return Ok(());
        }
        return Err(BiError::StringError(
            "Default AI provider not yet configured".to_string(),
        ));
    }
}
