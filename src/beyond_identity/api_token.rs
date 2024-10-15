use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
struct ApiTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct StoredToken {
    access_token: String,
    expires_at: u64,
    tenant_id: String,
    realm_id: String,
    application_id: String,
}

pub async fn get_beyond_identity_api_token(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<String, BiError> {
    let token_file_path = config.file_paths.token_path.clone();

    // Check if the token file exists and read the token if it does
    if let Ok(data) = fs::read_to_string(&token_file_path) {
        if let Ok(stored_token) = serde_json::from_str::<StoredToken>(&data) {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if stored_token.expires_at > current_time
                && stored_token.tenant_id == tenant_config.tenant_id
                && stored_token.realm_id == tenant_config.realm_id
                && stored_token.application_id == tenant_config.application_id
            {
                log::debug!("Using stored bearer token for all requests");
                return Ok(stored_token.access_token);
            }
        }
    }

    log::debug!("No valid token found. Fetching a new one.");

    // If no valid token, fetch a new one
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
        .basic_auth(
            tenant_config.client_id.clone(),
            Some(tenant_config.client_secret.clone()),
        )
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let token_response: ApiTokenResponse = serde_json::from_str(&response_text)?;

    // Calculate the expiration time
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expires_at = current_time + token_response.expires_in;

    // Save the token and expiration time to a file
    let stored_token = StoredToken {
        access_token: token_response.access_token.clone(),
        expires_at,
        tenant_id: tenant_config.tenant_id.clone(),
        realm_id: tenant_config.realm_id.clone(),
        application_id: tenant_config.application_id.clone(),
    };
    let serialized =
        serde_json::to_string(&stored_token).map_err(BiError::SerdeError)?;
    fs::write(token_file_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(token_file_path))?;

    Ok(token_response.access_token)
}
