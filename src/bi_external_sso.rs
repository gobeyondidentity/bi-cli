use crate::bi_api_token::get_beyond_identity_api_token;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticatorConfig {
    display_name: String,
    config: AuthenticatorConfigType,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticatorConfigType {
    #[serde(rename = "type")]
    config_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticatorConfigResponse {
    id: String,
    realm_id: String,
    tenant_id: String,
    display_name: String,
    config: AuthenticatorConfigType,
}

async fn create_authenticator_config(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<AuthenticatorConfigResponse, BiError> {
    let bi_api_token = get_beyond_identity_api_token(client, config, tenant_config).await?;

    let url = format!(
        "{}/v1/tenants/{}/realms/{}/authenticator-configs",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let payload = json!({
        "authenticator_config": {
            "display_name": "Platform Authenticator Configuration",
            "config": {
                "type": "platform"
            }
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", bi_api_token))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let auth_config_response: AuthenticatorConfigResponse = serde_json::from_str(&response_text)?;
    Ok(auth_config_response)
}

#[derive(Debug, Serialize, Deserialize)]
struct ApplicationConfig {
    display_name: String,
    authenticator_config: AuthenticatorConfigType,
    protocol_config: ProtocolConfig,
    classification: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolConfig {
    #[serde(rename = "type")]
    config_type: String,
    allowed_scopes: Vec<String>,
    confidentiality: String,
    grant_type: Vec<String>,
    pkce: String,
    redirect_uris: Vec<String>,
    token_configuration: TokenConfiguration,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenConfiguration {
    expires_after: i64,
    token_signing_algorithm: String,
    subject_field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSSO {
    pub id: String,
    realm_id: String,
    tenant_id: String,
    display_name: String,
    pub protocol_config: ProtocolConfig,
}

async fn create_application(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    auth_config_id: &str,
) -> Result<ExternalSSO, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/applications",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let payload = json!({
        "application": {
            "display_name": "Okta",
            "authenticator_config_id": auth_config_id,
            "protocol_config": {
                "type": "oidc",
                "allowed_scopes": [],
                "confidentiality": "confidential",
                "grant_type": ["authorization_code"],
                "pkce": "disabled",
                "redirect_uris": [],
                "token_configuration": {
                    "expires_after": 86400,
                    "token_signing_algorithm": "RS256",
                    "subject_field": "id"
                },
                "token_endpoint_auth_method": "client_secret_basic",
                "token_format": "self_contained"
            },
            "classification": "delegate_idp"
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                get_beyond_identity_api_token(client, config, tenant_config).await?
            ),
        )
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let app_config_response: ExternalSSO = serde_json::from_str(&response_text)?;
    Ok(app_config_response)
}

pub async fn load_external_sso(config: &Config) -> Result<ExternalSSO, BiError> {
    let config_path = config.file_paths.external_sso_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let external_sso_config: ExternalSSO =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(external_sso_config)
}

pub async fn create_external_sso(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<ExternalSSO, BiError> {
    let auth_config = create_authenticator_config(client, config, tenant_config).await?;
    let app_config = create_application(client, config, tenant_config, &auth_config.id).await?;
    let serialized = serde_json::to_string_pretty(&app_config)?;

    let config_path = config.file_paths.external_sso_config.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    Ok(app_config)
}
