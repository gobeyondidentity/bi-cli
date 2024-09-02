use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fs};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct OneLoginApp {
    id: i64,
    name: String,
    icon_url: String,
}

pub type AppsToMigrate = HashMap<String, String>;

pub type AppMapping = HashMap<String, String>;

pub async fn vitalsource_create_bookmarks(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_token: &str,
    bi_token: &str,
) -> Result<(), BiError> {
    let mut apps_to_migrate = AppsToMigrate::new();
    let mut app_mapping = AppMapping::new();

    if let Ok(data) = fs::read_to_string(&config.file_paths.vitalsource_apps_to_migrate) {
        if let Ok(existing_mapping) = serde_json::from_str::<AppsToMigrate>(&data) {
            apps_to_migrate = existing_mapping;
        }
    }

    let mut new_bookmarks_created = 0;

    // Loop through the app IDs and create a bookmark for each.
    for (app_id, app_name) in apps_to_migrate.iter() {
        let app = get_onelogin_app(client, config, onelogin_token, app_id).await?;

        match bi_create_bookmark(client, config, tenant_config, &bi_token, &app).await {
            Ok(bookmark_id) => {
                app_mapping.insert(app_id.to_string(), bookmark_id);
                new_bookmarks_created += 1;
            }
            Err(e) => {
                log::error!(
                    "Failed to create bookmark for app {} {}: {:?}",
                    app_id,
                    app_name,
                    e
                );
            }
        }
    }

    let serialized = serde_json::to_string_pretty(&app_mapping)?;
    let config_path = config.file_paths.vitalsource_app_mapping.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    log::info!("New bookmarks created: {}", new_bookmarks_created);

    Ok(())
}

async fn get_onelogin_app(
    client: &Client,
    config: &Config,
    onelogin_token: &str,
    app_id: &str,
) -> Result<OneLoginApp, BiError> {
    let url = format!("{}/api/2/apps/{}", config.onelogin_base_url, app_id);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", onelogin_token))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let app: OneLoginApp = serde_json::from_str(&response_text)?;
    Ok(app)
}
// Creates a Beyond Identity based on the OneLogin Role.
// Returns the ID of the group created in Beyond Identity.
async fn bi_create_bookmark(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    app: &OneLoginApp,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let login_link = format!("{}/launch/{}", config.onelogin_base_url, app.id);
    let sanitized_name = app
        .name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .take(64)
        .collect::<String>();

    let payload = json!({
        "sso_config": {
            "is_migrated": false,
            "display_name": sanitized_name,
            "payload": {
                "type": "bookmark",
                "login_link": login_link,
                "icon": app.icon_url,
                "is_tile_visible": true
            }
        }
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::info!("{} URL: {} and response: {}", status, url, response_text);

    if !status.is_success() {
        Err(BiError::RequestError(status, response_text))
    } else {
        let bookmark: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bookmark["id"].as_str().unwrap().to_string())
    }
}
