use crate::config::Config;
use crate::error::BiError;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantConfig {
    pub application_id: String,
    pub authenticator_config_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub console_login_link: String,
    pub identity_id: String,
    pub magic_link: String,
    pub open_id_configuration_url: String,
    pub realm_id: String,
    pub resource_server_id: String,
    pub tenant_id: String,
}

pub async fn create_tenant(client: &Client, config: &Config) -> Result<TenantConfig, BiError> {
    let tenant_display_name = format!("SW Rolling - {}", Utc::now().format("%Y%m%d-%H%M%S"));
    let payload = serde_json::json!({
        "tenant_display_name": tenant_display_name,
        "admin_display_name": config.admin_display_name,
        "admin_primary_email_address": config.admin_primary_email_address,
        "classification": "Secure Workforce"
    });
    let response = client
        .post(format!(
            "{}/v1/tenants",
            config.beyond_identity_api_base_url
        ))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    let json_response = response.json::<TenantConfig>().await?;
    Ok(json_response)
}

pub async fn load_tenant(config: &Config) -> Result<TenantConfig, BiError> {
    let config_path = config.file_paths.tenant_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let tenant_config: TenantConfig =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(tenant_config)
}
