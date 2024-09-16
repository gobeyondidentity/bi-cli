use crate::config::Config;
use crate::error::BiError;
use std::fs;
use std::io::{self, Write};

pub async fn provision_existing_tenant(config: &Config) -> Result<crate::tenant::TenantConfig, BiError> {
    // Prompt for tenant_id
    print!("Enter tenant_id: ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut tenant_id = String::new();
    io::stdin()
        .read_line(&mut tenant_id)
        .map_err(BiError::IoError)?;
    let tenant_id = tenant_id.trim().to_string();

    // Prompt for realm_id
    print!("Enter realm_id: ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut realm_id = String::new();
    io::stdin()
        .read_line(&mut realm_id)
        .map_err(BiError::IoError)?;
    let realm_id = realm_id.trim().to_string();

    // Prompt for application_id
    print!("Enter application_id: ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut application_id = String::new();
    io::stdin()
        .read_line(&mut application_id)
        .map_err(BiError::IoError)?;
    let application_id = application_id.trim().to_string();

    // Prompt for client_id
    print!("Enter client_id: ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut client_id = String::new();
    io::stdin()
        .read_line(&mut client_id)
        .map_err(BiError::IoError)?;
    let client_id = client_id.trim().to_string();

    // Prompt for client_secret
    print!("Enter client_secret: ");
    io::stdout().flush().map_err(BiError::IoError)?;
    let mut client_secret = String::new();
    io::stdin()
        .read_line(&mut client_secret)
        .map_err(BiError::IoError)?;
    let client_secret = client_secret.trim().to_string();

    // Create the tenant configuration
    let tenant_config = crate::tenant::TenantConfig {
        application_id,
        client_id,
        client_secret,
        realm_id: realm_id.clone(),
        tenant_id: tenant_id.clone(),
        authenticator_config_id: None,
        console_login_link: None,
        identity_id: None,
        magic_link: None,
        open_id_configuration_url: None,
        resource_server_id: None,
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
