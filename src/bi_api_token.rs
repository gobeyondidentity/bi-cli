use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiTokenResponse {
    access_token: String,
}

pub async fn get_beyond_identity_api_token(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/applications/{}/token",
        config.beyond_identity_auth_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        tenant_config.application_id
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(tenant_config.client_id.clone(), Some(tenant_config.client_secret.clone()))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let token_response: ApiTokenResponse = serde_json::from_str(&response_text)?;
    Ok(token_response.access_token)
}
