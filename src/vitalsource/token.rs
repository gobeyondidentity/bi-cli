use crate::config::Config;
use crate::error::BiError;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde_json::json;

pub async fn get_onelogin_access_token(
    client: &Client,
    config: &Config,
) -> Result<String, BiError> {
    let url = format!("{}/auth/oauth2/v2/token", config.onelogin_base_url);
    let payload = json!({
        "grant_type": "client_credentials",
        "client_id": config.onelogin_client_id,
        "client_secret": config.onelogin_client_secret
    });

    let response = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await?;

    let token_response: serde_json::Value = response.json().await?;
    Ok(token_response["access_token"].as_str().unwrap().to_string())
}

pub async fn get_entra_access_token(client: &Client, config: &Config) -> Result<String, BiError> {
    let url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        config.entra_tenant_id
    );

    let response = client
        .post(url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", &config.entra_client_id),
            ("client_secret", &config.entra_client_secret),
            ("scope", "https://graph.microsoft.com/.default"),
        ])
        .send()
        .await?;

    let token_response: serde_json::Value = response.json().await?;
    Ok(token_response["access_token"].as_str().unwrap().to_string())
}
