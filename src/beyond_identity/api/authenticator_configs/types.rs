use clap::{Args, Subcommand, ValueEnum};
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Authenticator Config Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct AuthenticatorConfigs {
    pub authenticator_configs: Vec<AuthenticatorConfig>,
    pub total_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorConfigEnvelope {
    pub authenticator_config: AuthenticatorConfig,
}

/// Representation of an authenticator configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorConfig {
    /// A unique identifier for the authenticator config.
    pub id: String,

    /// A unique identifier for the realm associated with this authenticator config.
    pub realm_id: String,

    /// A unique identifier for the tenant associated with this authenticator config.
    pub tenant_id: String,

    /// A human-readable name for the authenticator configuration.
    pub display_name: Option<String>,

    /// Configuration details for the authenticator.
    pub config: AuthenticatorConfigDetails,
}

/// Enum representing the details of the authenticator configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthenticatorConfigDetails {
    /// Embedded SDK authenticator configuration.
    Embedded(EmbeddedAuthenticatorConfig),

    /// Hosted web authenticator configuration.
    HostedWeb(HostedWebAuthenticatorConfig),

    /// Platform authenticator configuration.
    Platform(PlatformAuthenticatorConfig),
}

/// Configuration options for the embedded SDK authenticator.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedAuthenticatorConfig {
    /// URL to invoke during the authentication flow.
    pub invoke_url: String,

    /// The method used to invoke the `invoke_url` in the embedded authenticator config type.
    pub invocation_type: InvocationType,

    /// Set of authentication methods that are available to the authenticator.
    pub authentication_methods: Vec<AuthenticationMethod>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the hosted web experience authenticator.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostedWebAuthenticatorConfig {
    /// Set of authentication methods that are available to the authenticator.
    pub authentication_methods: Vec<AuthenticationMethod>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalAuthenticatorConfigRequest {
    pub authenticator_config: OptionalAuthenticatorConfig,
}

/// Representation of an authenticator configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalAuthenticatorConfig {
    /// A unique identifier for the authenticator config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A unique identifier for the realm associated with this authenticator config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_id: Option<String>,

    /// A unique identifier for the tenant associated with this authenticator config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// A human-readable name for the authenticator configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Configuration details for the authenticator.
    pub config: OptionalAuthenticatorConfigDetails,
}

/// Enum representing the details of the authenticator configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OptionalAuthenticatorConfigDetails {
    /// Embedded SDK authenticator configuration.
    Embedded(OptionalEmbeddedAuthenticatorConfig),

    /// Hosted web authenticator configuration.
    HostedWeb(OptionalHostedWebAuthenticatorConfig),

    /// Platform authenticator configuration.
    Platform(OptionalPlatformAuthenticatorConfig),
}

/// Configuration options for the embedded SDK authenticator.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalEmbeddedAuthenticatorConfig {
    /// URL to invoke during the authentication flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_url: Option<String>,

    /// The method used to invoke the `invoke_url` in the embedded authenticator config type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<InvocationType>,

    /// Set of authentication methods that are available to the authenticator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_methods: Option<Vec<AuthenticationMethod>>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the hosted web experience authenticator.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalHostedWebAuthenticatorConfig {
    /// Set of authentication methods that are available to the authenticator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_methods: Option<Vec<AuthenticationMethod>>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the platform authenticator.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalPlatformAuthenticatorConfig {
    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAuthenticatorConfigRequest {
    #[clap(flatten)]
    pub authenticator_config: CreateAuthenticatorConfig,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAuthenticatorConfig {
    /// Configuration details for the authenticator.
    #[clap(subcommand)]
    pub config: CreateAuthenticatorConfigDetails,
}

/// Enum representing the details of the authenticator configuration.
#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateAuthenticatorConfigDetails {
    /// Embedded SDK authenticator configuration.
    Embedded(CreateEmbeddedAuthenticatorConfig),

    /// Hosted web authenticator configuration.
    HostedWeb(CreateHostedWebAuthenticatorConfig),

    /// Platform authenticator configuration.
    Platform(CreatePlatformAuthenticatorConfig),
}

/// Configuration options for the embedded SDK authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct CreateEmbeddedAuthenticatorConfig {
    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    pub display_name: Option<String>,

    /// URL to invoke during the authentication flow.
    #[clap(long)]
    pub invoke_url: String,

    /// The method used to invoke the `invoke_url` in the embedded authenticator config type.
    #[clap(long, value_enum)]
    pub invocation_type: InvocationType,

    /// Set of authentication methods that are available to the authenticator.
    #[clap(long, use_value_delimiter = true, num_args(1..), required = true)]
    pub authentication_methods: Vec<AuthenticationMethod>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the hosted web experience authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct CreateHostedWebAuthenticatorConfig {
    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    pub display_name: Option<String>,

    /// Set of authentication methods that are available to the authenticator.
    #[clap(long, use_value_delimiter = true, num_args(1..), required = true)]
    pub authentication_methods: Vec<AuthenticationMethod>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the platform authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct CreatePlatformAuthenticatorConfig {
    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    pub display_name: Option<String>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

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

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct PatchAuthenticatorConfigRequest {
    #[clap(flatten)]
    pub authenticator_config: PatchAuthenticatorConfig,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct PatchAuthenticatorConfig {
    /// Configuration details for the authenticator.
    #[clap(subcommand)]
    pub config: PatchAuthenticatorConfigDetails,
}

/// Enum representing the details of the authenticator configuration.
#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PatchAuthenticatorConfigDetails {
    /// Embedded SDK authenticator configuration.
    Embedded(PatchEmbeddedAuthenticatorConfig),

    /// Hosted web authenticator configuration.
    HostedWeb(PatchHostedWebAuthenticatorConfig),

    /// Platform authenticator configuration.
    Platform(PatchPlatformAuthenticatorConfig),
}

/// Configuration options for the embedded SDK authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct PatchEmbeddedAuthenticatorConfig {
    /// A unique identifier for the authenticator config.
    #[clap(long)]
    pub id: String,

    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// URL to invoke during the authentication flow.
    #[clap(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_url: Option<String>,

    /// The method used to invoke the `invoke_url` in the embedded authenticator config type.
    #[clap(long, value_enum)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<InvocationType>,

    /// Set of authentication methods that are available to the authenticator.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_methods: Option<Vec<AuthenticationMethod>>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the hosted web experience authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct PatchHostedWebAuthenticatorConfig {
    /// A unique identifier for the authenticator config.
    #[clap(long)]
    pub id: String,

    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Set of authentication methods that are available to the authenticator.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_methods: Option<Vec<AuthenticationMethod>>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

/// Configuration options for the platform authenticator.
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct PatchPlatformAuthenticatorConfig {
    /// A unique identifier for the authenticator config.
    #[clap(long)]
    pub id: String,

    /// A human-readable name for the authenticator configuration.
    #[clap(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Trusted origins are URLs that will be allowed to make requests from a browser to the Beyond Identity API.
    #[clap(long, use_value_delimiter = true, num_args(0..))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_origins: Option<Vec<String>>,
}

impl From<&CreateAuthenticatorConfigRequest> for OptionalAuthenticatorConfigRequest {
    fn from(request: &CreateAuthenticatorConfigRequest) -> Self {
        let CreateAuthenticatorConfig { config } = request.authenticator_config.clone();

        let authenticator_config = match config {
            CreateAuthenticatorConfigDetails::Embedded(cfg) => OptionalAuthenticatorConfig {
                id: None,
                realm_id: None,
                tenant_id: None,
                display_name: cfg.display_name,
                config: OptionalAuthenticatorConfigDetails::Embedded(
                    OptionalEmbeddedAuthenticatorConfig {
                        invoke_url: Some(cfg.invoke_url),
                        invocation_type: Some(cfg.invocation_type),
                        authentication_methods: Some(cfg.authentication_methods),
                        trusted_origins: cfg.trusted_origins,
                    },
                ),
            },
            CreateAuthenticatorConfigDetails::HostedWeb(cfg) => OptionalAuthenticatorConfig {
                id: None,
                realm_id: None,
                tenant_id: None,
                display_name: cfg.display_name,
                config: OptionalAuthenticatorConfigDetails::HostedWeb(
                    OptionalHostedWebAuthenticatorConfig {
                        authentication_methods: Some(cfg.authentication_methods),
                        trusted_origins: cfg.trusted_origins,
                    },
                ),
            },
            CreateAuthenticatorConfigDetails::Platform(cfg) => OptionalAuthenticatorConfig {
                id: None,
                realm_id: None,
                tenant_id: None,
                display_name: cfg.display_name,
                config: OptionalAuthenticatorConfigDetails::Platform(
                    OptionalPlatformAuthenticatorConfig {
                        trusted_origins: cfg.trusted_origins,
                    },
                ),
            },
        };

        OptionalAuthenticatorConfigRequest {
            authenticator_config,
        }
    }
}

impl From<&PatchAuthenticatorConfigRequest> for OptionalAuthenticatorConfigRequest {
    fn from(request: &PatchAuthenticatorConfigRequest) -> Self {
        let PatchAuthenticatorConfig { config } = request.authenticator_config.clone();

        let authenticator_config = match config {
            PatchAuthenticatorConfigDetails::Embedded(cfg) => OptionalAuthenticatorConfig {
                id: Some(cfg.id),
                realm_id: None,
                tenant_id: None,
                display_name: cfg.display_name,
                config: OptionalAuthenticatorConfigDetails::Embedded(
                    OptionalEmbeddedAuthenticatorConfig {
                        invoke_url: cfg.invoke_url,
                        invocation_type: cfg.invocation_type,
                        authentication_methods: cfg.authentication_methods,
                        trusted_origins: cfg.trusted_origins,
                    },
                ),
            },
            PatchAuthenticatorConfigDetails::HostedWeb(cfg) => OptionalAuthenticatorConfig {
                id: Some(cfg.id),
                realm_id: None,
                tenant_id: None,
                display_name: cfg.display_name,
                config: OptionalAuthenticatorConfigDetails::HostedWeb(
                    OptionalHostedWebAuthenticatorConfig {
                        authentication_methods: cfg.authentication_methods,
                        trusted_origins: cfg.trusted_origins,
                    },
                ),
            },
            PatchAuthenticatorConfigDetails::Platform(cfg) => OptionalAuthenticatorConfig {
                id: Some(cfg.id),
                realm_id: None,
                tenant_id: None,
                display_name: cfg.display_name,
                config: OptionalAuthenticatorConfigDetails::Platform(
                    OptionalPlatformAuthenticatorConfig {
                        trusted_origins: cfg.trusted_origins,
                    },
                ),
            },
        };

        OptionalAuthenticatorConfigRequest {
            authenticator_config,
        }
    }
}
