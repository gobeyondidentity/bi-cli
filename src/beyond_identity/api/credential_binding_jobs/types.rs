use crate::beyond_identity::api::authenticator_configs::types::AuthenticatorConfig;

use clap::{ArgGroup, Args, ValueEnum};
use field_types::FieldName;
use serde::{Deserialize, Serialize};

// ====================================
// Credential Binding Job Types
// ====================================

#[derive(Clone, Debug, Serialize, Deserialize, FieldName)]
pub struct CredentialBindingJobs {
    pub credential_binding_jobs: Vec<CredentialBindingJob>,
    pub total_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CredentialBindingJobEnvelope {
    pub credential_binding_job: CredentialBindingJob,
    pub credential_binding_link: Option<String>,
}

/// The method by which a credential binding link is delivered to the target authenticator or identity.
#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryMethod {
    /// Indicates that a credential binding link will be returned to the caller upon creation of the credential binding job
    Return,
    /// Indicates that a credential binding link will be sent to the email address associated with the identity.
    Email,
}

/// The current state of the credential binding job.
#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    /// Indicates that the credential binding request has been successfully delivered to its target authenticator.
    RequestDelivered,
    /// Indicates that the credential binding link associated with the job was sent to its target authenticator or identity.
    LinkSent,
    /// Indicates that the credential binding link associated with the job was opened by its target identity.
    LinkOpened,
    /// Indicates that a credential was successfully bound to an identity.
    Complete,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCredentialBindingJobRequest {
    #[clap(flatten)]
    job: CreateCredentialBindingJob,
}

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("authenticator_config_group").required(true).args(&["authenticator_config", "authenticator_config_id"]).multiple(false))]
pub struct CreateCredentialBindingJob {
    /// (required) The method by which a credential binding link is delivered to the target authenticator or identity.
    #[clap(long, value_enum)]
    pub delivery_method: DeliveryMethod,
    /// (optional) The URI to which the caller will be redirected after successfully binding a credential to an identity.
    #[clap(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_binding_redirect_uri: Option<String>,
    /// The full authenticator configuration (optional if `authenticator_config_id` is provided).
    ///
    /// Example JSON for an embedded authenticator configuration:
    /// {
    ///   "config": {
    ///     "type": "embedded",
    ///     "invoke_url": "https://example.com/authenticate",
    ///     "invocation_type": "automatic",
    ///     "authentication_methods": [{"type": "webauthn_passkey"}, {"type": "software_passkey"}],
    ///     "trusted_origins": ["https://trusted-origin1.com", "https://trusted-origin2.com"]
    ///   }
    /// }
    ///
    /// Example JSON for a platform authenticator configuration:
    /// {
    ///   "config": {
    ///     "type": "platform",
    ///     "trusted_origins": ["https://trusted-origin.com"]
    ///   }
    /// }
    #[clap(long, group = "authenticator_config_group", value_parser = clap::builder::ValueParser::new(|s: &str| serde_json::from_str::<AuthenticatorConfig>(s).map_err(|e| e.to_string())))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_config: Option<AuthenticatorConfig>,
    /// The ID of the authenticator configuration (optional if `authenticator_config` is provided).
    #[clap(long, group = "authenticator_config_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_config_id: Option<String>,
}

/// A credential binding job defines the state of binding a new credential to an identity. The state includes creation of the credential binding job to delivery of the credential binding method to completion of the credential binding.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CredentialBindingJob {
    /// A unique identifier for the credential binding job.
    pub id: String,

    /// A unique identifier for the realm associated with this job.
    pub realm_id: String,

    /// A unique identifier for the tenant associated with this job.
    pub tenant_id: String,

    /// A unique identifier for the identity to which the credential will be bound.
    pub identity_id: String,

    /// The method by which a credential binding link is delivered to the target authenticator or identity.
    pub delivery_method: DeliveryMethod,

    /// The current state of the credential binding job. This field is optional.
    pub state: Option<State>,

    /// The URI to which the caller will be redirected after successfully binding a credential to an identity.
    pub post_binding_redirect_uri: Option<String>,

    /// The full authenticator configuration (optional if `authenticator_config_id` is provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_config: Option<AuthenticatorConfig>,

    /// The ID of the authenticator configuration (optional if `authenticator_config` is provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_config_id: Option<String>,

    /// The time at which this credential binding job will expire, represented as an ISO 8601 string.
    pub expire_time: String,

    /// The time at which this credential binding job was created, represented as an ISO 8601 string.
    pub create_time: String,

    /// The time at which this credential binding job was last updated, represented as an ISO 8601 string.
    pub update_time: String,
}
