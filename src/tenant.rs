use crate::config::Config;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

pub async fn create_tenant(
    client: &Client,
    config: &Config,
) -> Result<TenantConfig, reqwest::Error> {
    let tenant_display_name = format!("SW Rolling - {}", Utc::now().format("%Y%m%d-%H%M%S"));
    let payload = serde_json::json!({
        "tenant_display_name": tenant_display_name,
        "admin_display_name": config.admin_display_name,
        "admin_primary_email_address": config.admin_primary_email_address,
        "classification": "Secure Workforce"
    });
    let response = client
        .post(format!("{}/v1/tenants", config.beyond_identity_api_base_url))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    let json_response = response.json::<TenantConfig>().await?;
    Ok(json_response)
}

pub async fn load_or_create_tenant(
    client: &Client,
    config: &Config,
) -> TenantConfig {
    let config_path = config.file_paths.tenant_config.clone();
    if Path::new(&config_path).exists() {
        let data = fs::read_to_string(config_path).expect("Unable to read file");
        serde_json::from_str(&data).expect("JSON was not well-formatted")
    } else {
        let tenant_response = create_tenant(client, config)
            .await
            .expect("Failed to create tenant");
        let serialized =
            serde_json::to_string_pretty(&tenant_response).expect("Failed to serialize tenant response");
        fs::write(config_path, serialized).expect("Unable to write file");
        tenant_response
    }
}
