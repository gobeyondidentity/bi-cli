use crate::beyond_identity::api_token::get_beyond_identity_api_token;
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use rand::Rng;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaApplication {
    id: String,
    pub label: String,
    status: String,
    // sign_on_mode: String,
    // settings: OktaApplicationSettings,
    embedded: Option<OktaEmbeddedUsers>,
    #[serde(rename = "_links")]
    _links: Links,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Links {
    upload_logo: UploadLogo,
    app_links: Vec<AppLink>,
    logo: Vec<Logo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadLogo {
    href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppLink {
    name: String,
    href: String,
    r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Logo {
    name: String,
    href: String,
    r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OktaApplicationSettings {
    app: OktaApplicationSettingsApp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OktaApplicationSettingsApp {
    login_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OktaEmbeddedUsers {
    users: Vec<OktaUser>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OktaUser {
    id: String,
    profile: OktaUserProfile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OktaUserProfile {
    email: Option<String>,
}

async fn fetch_all_okta_users(
    client: &Client,
    config: &Config,
) -> Result<HashMap<String, OktaUser>, BiError> {
    let mut users_map = HashMap::new();
    let mut url = format!("{}/api/v1/users?limit=200", config.okta_domain);
    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", config.okta_api_key))
            .send()
            .await?;

        let status = response.status();
        log::debug!("{} response status: {}", url, status);
        let next_link = extract_next_link(&response);
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let response_text = response.text().await?;
        log::debug!("{} response text: {}", url, response_text);
        let page_users: Vec<OktaUser> = serde_json::from_str(&response_text)?;

        for user in page_users {
            users_map.insert(user.id.clone(), user);
        }

        if let Some(next) = next_link {
            url = next;
        } else {
            break;
        }
    }

    Ok(users_map)
}

pub async fn fetch_okta_applications(
    client: &Client,
    config: &Config,
) -> Result<Vec<OktaApplication>, BiError> {
    let mut apps = Vec::new();
    let mut url = format!(
        "{}/api/v1/apps?limit=200&filter=status eq \"ACTIVE\"",
        config.okta_domain
    );

    let users_map = fetch_all_okta_users(client, config).await?;

    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", config.okta_api_key))
            .send()
            .await?;

        let status = response.status();
        log::debug!("{} response status: {}", url, status);
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let next_link = extract_next_link(&response);
        let response_text = response.text().await?;
        log::debug!("{} response text: {}", url, response_text);
        let mut page_apps: Vec<OktaApplication> = serde_json::from_str(&response_text)?;

        for app in &mut page_apps {
            log::info!("Fetching assigned users for app: {:?}", app.label);
            let users = get_users_assigned_to_app(client, config, &app.id, &users_map).await?;
            app.embedded = Some(OktaEmbeddedUsers { users });
            let sleep_duration = rand::thread_rng().gen_range(2..=4);
            sleep(Duration::from_secs(sleep_duration)).await;
        }

        apps.extend(page_apps);

        if let Some(next) = next_link {
            url = next;
        } else {
            break;
        }
    }

    let serialized = serde_json::to_string_pretty(&apps)?;

    let config_path = config.file_paths.okta_applications.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    Ok(apps)
}

fn extract_next_link(response: &reqwest::Response) -> Option<String> {
    response.headers().get_all("link").iter().find_map(|link| {
        let link_str = link.to_str().ok()?;
        if link_str.contains("rel=\"next\"") {
            Some(
                link_str
                    .split(';')
                    .next()?
                    .trim()
                    .trim_matches('<')
                    .trim_matches('>')
                    .to_string(),
            )
        } else {
            None
        }
    })
}

pub async fn load_okta_applications(config: &Config) -> Result<Vec<OktaApplication>, BiError> {
    let config_path = config.file_paths.okta_applications.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let okta_applications: Vec<OktaApplication> =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(okta_applications)
}

async fn get_users_assigned_to_app(
    client: &Client,
    config: &Config,
    app_id: &str,
    users_map: &HashMap<String, OktaUser>,
) -> Result<Vec<OktaUser>, BiError> {
    let mut all_users = Vec::new();
    let mut url = format!(
        "{}/api/v1/apps/{}/users?limit=450",
        config.okta_domain, app_id
    );

    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", config.okta_api_key))
            .send()
            .await?;

        let status = response.status();
        let next_link = extract_next_link(&response);
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

        let mut users: Vec<OktaUser> = serde_json::from_str(&response_text)?;

        // Backfill missing emails
        for user in &mut users {
            if user.profile.email.is_none() {
                if let Some(full_user) = users_map.get(&user.id) {
                    user.profile.email = full_user.profile.email.clone();
                }
            }
        }

        println!("Fetched {} users for app {}", users.len(), app_id);
        all_users.extend(users);

        // Check for next link
        if let Some(next) = next_link {
            url = next;
            println!("Fetching next page: {}", url);
        } else {
            break;
        }
    }

    println!(
        "Total users fetched for app {}: {}",
        app_id,
        all_users.len()
    );
    Ok(all_users)
}

pub fn select_applications(applications: &[OktaApplication]) -> Vec<OktaApplication> {
    println!("Select applications to fast migrate (comma separated indices or 'all' for all applications):");

    for (index, app) in applications.iter().enumerate() {
        println!("{}: {} - {} ({})", index, app.label, app.id, app.status);
    }

    print!("Your selection: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "all" {
        return applications.to_vec();
    }

    let indices: Vec<usize> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    indices
        .into_iter()
        .map(|i| applications[i].clone())
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsoConfig {
    id: String,
    display_name: String,
    is_migrated: bool,
    payload: SsoConfigPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsoConfigPayload {
    #[serde(rename = "Bookmark")]
    bookmark: Bookmark,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    login_link: String,
    icon: String,
    is_tile_visible: bool,
    application_tile_id: String,
}

async fn create_sso_config(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    okta_application: &OktaApplication,
) -> Result<SsoConfig, BiError> {
    let bi_api_token = get_beyond_identity_api_token(client, config, tenant_config).await?;
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let display_name = sanitize_label(&okta_application.label);
    let login_link = okta_application
        ._links
        .app_links
        .get(0)
        .ok_or_else(|| BiError::StringError("No app_link present".to_string()))?;
    let logo = okta_application
        ._links
        .logo
        .get(0)
        .cloned()
        .unwrap_or(Logo {
            name: "default".to_string(),
            href: "https://static.byndid.com/logos/beyondidentity.png".to_string(),
            r#type: "image/png".to_string(),
        });

    let payload = json!({
        "sso_config": {
            "display_name": display_name,
            "is_migrated": true,
            "payload": {
                "type": "bookmark",
                "login_link": login_link.href,
                "icon": logo.href,
                "is_tile_visible": true
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

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let sso_config: SsoConfig = serde_json::from_str(&response_text)?;
    Ok(sso_config)
}

fn sanitize_label(label: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z\s]").unwrap();
    let sanitized_label: String = re.replace_all(label, "").to_string();
    let trimmed_label = sanitized_label.trim();
    if trimmed_label.len() > 60 {
        trimmed_label[..60].to_string()
    } else {
        trimmed_label.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    id: String,
    realm_id: String,
    tenant_id: String,
    display_name: String,
    create_time: String,
    update_time: String,
    traits: IdentityTraits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityTraits {
    username: String,
    primary_email_address: Option<String>,
}

pub async fn fetch_beyond_identity_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let mut identities = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?page_size=200",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    loop {
        let response = client
            .get(&url)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    get_beyond_identity_api_token(client, config, tenant_config).await?
                ),
            )
            .send()
            .await?;

        let status = response.status();
        log::debug!("{} response status: {}", url, status);
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let response_text = response.text().await?;
        log::debug!("{} response text: {}", url, response_text);
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let page_identities: Vec<Identity> =
            serde_json::from_value(response_json["identities"].clone())?;

        identities.extend(page_identities);

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/identities?page_size=200&page_token={}",
                config.beyond_identity_api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(identities)
}

fn filter_identities(
    okta_users: &[OktaUser],
    beyond_identity_identities: &[Identity],
) -> Vec<Identity> {
    let okta_user_emails: Vec<&str> = okta_users
        .iter()
        .filter_map(|user| user.profile.email.as_deref())
        .collect();
    beyond_identity_identities
        .iter()
        .filter(|identity| {
            identity
                .traits
                .primary_email_address
                .as_deref()
                .map_or(false, |email| okta_user_emails.contains(&email))
        })
        .cloned()
        .collect()
}

async fn assign_identities_to_sso_config(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    sso_config: &SsoConfig,
    identities: &[Identity],
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs/{}:addIdentities",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        sso_config.id
    );

    let identity_ids: Vec<String> = identities
        .iter()
        .map(|identity| identity.id.clone())
        .collect();
    let payload = json!({
        "identity_ids": identity_ids,
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

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    Ok(())
}

pub async fn create_sso_config_and_assign_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    okta_application: &OktaApplication,
) -> Result<SsoConfig, BiError> {
    let sso_config = create_sso_config(client, config, tenant_config, okta_application).await?;

    let beyond_identity_identities =
        fetch_beyond_identity_identities(client, config, tenant_config).await?;
    let filtered_identities = filter_identities(
        &okta_application.embedded.as_ref().unwrap().users,
        &beyond_identity_identities,
    );

    assign_identities_to_sso_config(
        client,
        config,
        tenant_config,
        &sso_config,
        &filtered_identities,
    )
    .await?;

    Ok(sso_config)
}

// Delete All SSO Configs

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSsoConfig {
    id: String,
    display_name: String,
}

async fn list_all_sso_configs(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<DeleteSsoConfig>, BiError> {
    let mut sso_configs = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs?page_size=100",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );
    let beyond_identity_api_token =
        get_beyond_identity_api_token(client, config, tenant_config).await?;

    loop {
        let response = client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", beyond_identity_api_token),
            )
            .send()
            .await?;

        let status = response.status();
        log::debug!("{} response status: {}", url, status);
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let response_text = response.text().await?;
        log::debug!("{} response text: {}", url, response_text);
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let page_sso_configs: Vec<DeleteSsoConfig> =
            serde_json::from_value(response_json["sso_configs"].clone())?;

        sso_configs.extend(page_sso_configs);

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/sso-configs?page_size=100&page_token={}",
                config.beyond_identity_api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(sso_configs)
}

async fn delete_sso_config(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    sso_config_id: &str,
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs/{}",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        sso_config_id
    );

    let response = client
        .delete(&url)
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                get_beyond_identity_api_token(client, config, tenant_config).await?
            ),
        )
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(BiError::RequestError(status, error_text));
    }

    Ok(())
}

pub async fn delete_all_sso_configs(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<(), BiError> {
    let sso_configs = list_all_sso_configs(client, config, tenant_config).await?;

    for sso_config in sso_configs {
        log::info!("Deleting SSO Config: {:?}", sso_config.id);
        delete_sso_config(client, config, tenant_config, &sso_config.id).await?;
    }

    Ok(())
}
