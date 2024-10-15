use crate::common::config::Config;
use crate::common::error::BiError;
use chrono::Utc;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use webbrowser::{open_browser, Browser};

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantConfig {
    pub application_id: String,
    pub authenticator_config_id: Option<String>,
    pub client_id: String,
    pub client_secret: String,
    pub console_login_link: Option<String>,
    pub identity_id: Option<String>,
    pub magic_link: Option<String>,
    pub open_id_configuration_url: Option<String>,
    pub realm_id: String,
    pub resource_server_id: Option<String>,
    pub tenant_id: String,
}

pub async fn create_tenant(client: &Client, config: &Config) -> Result<TenantConfig, BiError> {
    let tenant_display_name = format!("SA - {}", Utc::now().format("%Y%m%d-%H%M%S"));
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
    let tenant_config = response.json::<TenantConfig>().await?;

    let serialized = serde_json::to_string_pretty(&tenant_config)?;

    let config_path = config.file_paths.tenant_config.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    Ok(tenant_config)
}

pub async fn load_tenant(config: &Config) -> Result<TenantConfig, BiError> {
    let config_path = config.file_paths.tenant_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let tenant_config: TenantConfig =
        serde_json::from_str(&data).map_err(BiError::SerdeError)?;
    Ok(tenant_config)
}

pub fn open_magic_link(magic_link: &str) {
    println!("To finish setting up tenant creation, select a browser:");
    println!("1. Default");
    println!("2. Firefox");
    println!("3. Chrome");
    println!("4. Safari");
    println!("5. Show me the URL, I'll paste it into a browser myself");
    print!("Enter the number of your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => open_browser(Browser::Default, magic_link),
        "2" => open_browser(Browser::Firefox, magic_link),
        "3" => open_browser(Browser::Chrome, magic_link),
        "4" => open_browser(Browser::Safari, magic_link),
        "5" => {
            println!("Here is the magic link: {}", magic_link);
            return;
        }
        _ => {
            println!("Invalid choice, defaulting to system's default browser.");
            open_browser(Browser::Default, magic_link)
        }
    }
    .map(|_| println!("Opened magic link in selected browser."))
    .unwrap_or_else(|_| println!("Failed to open magic link in the selected browser."));
}
