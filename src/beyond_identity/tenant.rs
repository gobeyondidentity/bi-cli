use crate::common::config::Config;
use crate::common::error::BiError;
use chrono::Utc;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantConfig {
    pub tenant_id: String,
    pub realm_id: String,
    pub application_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub open_id_configuration_url: String,
    pub auth_base_url: String,
    pub api_base_url: String,
}

pub async fn load_tenant(config: &Config) -> Result<TenantConfig, BiError> {
    let config_path = config.file_paths.tenant_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let tenant_config: TenantConfig = serde_json::from_str(&data).map_err(BiError::SerdeError)?;
    Ok(tenant_config)
}

pub async fn provision_tenant(config: &Config) -> Result<TenantConfig, BiError> {
    // Prompt for issuer URL
    print!("Enter the issuer URL (Found in your Beyond Identity Management API application. In Secure Workforce, this is under API Access. In Secure Customer, it's under Applications): ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut issuer_url = String::new();
    io::stdin()
        .read_line(&mut issuer_url)
        .map_err(BiError::IoError)?;

    // Parse the URL and extract tenant_id, realm_id, application_id
    let parsed_url = Url::parse(&issuer_url).map_err(BiError::InvalidUrl)?;

    let segments: Vec<&str> = parsed_url
        .path_segments()
        .map_or(vec![], |segments| segments.collect());

    let tenant_id = segments
        .get(2)
        .ok_or(BiError::StringError("Invalid tenant ID".to_string()))?
        .to_string();
    let realm_id = segments
        .get(4)
        .ok_or(BiError::StringError("Invalid realm ID".to_string()))?
        .to_string();
    let application_id = segments
        .get(6)
        .ok_or(BiError::StringError("Invalid application ID".to_string()))?
        .to_string();

    // Extract base URLs from the issuer URL
    let host = parsed_url
        .host_str()
        .ok_or(BiError::StringError("Invalid URL host".to_string()))?;

    let auth_base_url = format!("https://{}", host);
    let api_base_url = host.replace("auth", "api");

    // Prompt for client_id
    print!("Enter the client ID (Located under External Protocol in your Beyond Identity Management API application): ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut client_id = String::new();
    io::stdin()
        .read_line(&mut client_id)
        .map_err(BiError::IoError)?;
    let client_id = client_id.trim().to_string();

    // Prompt for client_secret
    print!("Enter the client Secret (Located under External Protocol in your Beyond Identity Management API application): ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut client_secret = String::new();
    io::stdin()
        .read_line(&mut client_secret)
        .map_err(BiError::IoError)?;
    let client_secret = client_secret.trim().to_string();

    // Create the tenant configuration
    let tenant_config = TenantConfig {
        tenant_id,
        realm_id,
        application_id,
        client_id,
        client_secret,
        open_id_configuration_url: format!("{}/.well-known/openid-configuration", issuer_url),
        api_base_url,
        auth_base_url,
    };

    // Serialize the tenant configuration to JSON
    let serialized = serde_json::to_string_pretty(&tenant_config).map_err(BiError::SerdeError)?;

    // Ensure the configuration directory exists
    let config_path = config.file_paths.tenant_config.clone();
    let config_dir = std::path::Path::new(&config_path)
        .parent()
        .ok_or_else(|| BiError::UnableToWriteFile(config_path.clone()))?;
    fs::create_dir_all(config_dir).map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

    // Write the JSON payload to the specified tenant_config path
    fs::write(&config_path, serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

    println!("Tenant configuration saved to '{}'", config_path);

    Ok(tenant_config)
}
