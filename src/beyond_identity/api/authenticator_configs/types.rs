use clap::{Args, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

// ====================================
// Authenticator Config Types
// ====================================

/// Enum representing the possible types of authentication methods.
#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthenticationMethod {
    WebauthnPasskey,
    SoftwarePasskey,
    EmailOneTimePassword,
}

/// Enum representing the invocation type.
#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum InvocationType {
    Automatic,
    Manual,
}

/// Configuration options for the embedded SDK authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedAuthenticatorConfig {
    /// URL to invoke during the authentication flow.
    #[clap(long)]
    pub invoke_url: String,
    /// The method used to invoke the `invoke_url` in the embedded authenticator config type.
    #[clap(long, value_enum)]
    pub invocation_type: InvocationType,
    /// Set of authentication methods that are available to the authenticator.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    pub authentication_methods: Vec<AuthenticationMethod>,
    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the hosted web experience authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct HostedWebAuthenticatorConfig {
    /// Set of authentication methods that are available to the authenticator.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    pub authentication_methods: Vec<AuthenticationMethod>,
    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the platform authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct PlatformAuthenticatorConfig {
    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Enum representing the details of the authenticator configuration.
#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthenticatorConfigDetails {
    /// Embedded SDK authenticator configuration.
    Embedded(EmbeddedAuthenticatorConfig),
    /// Hosted web authenticator configuration.
    HostedWeb(HostedWebAuthenticatorConfig),
    /// Platform authenticator configuration.
    Platform(PlatformAuthenticatorConfig),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorConfigEnvelope {
    pub authenticator_config: AuthenticatorConfig,
}

/// Representation of an authenticator configuration.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorConfig {
    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Configuration details for the authenticator.
    #[clap(subcommand)]
    pub config: AuthenticatorConfigDetails,
}
