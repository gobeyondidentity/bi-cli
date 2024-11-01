use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::service::IdentitiesService;
use crate::beyond_identity::api::identities::api::IdentitiesApi;
use crate::beyond_identity::api::identities::types::Identity;
use crate::beyond_identity::helper::sso_configs;
use crate::common::database::models::OktaConfig;
use crate::common::error::BiError;

use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaApplication {
    id: String,
    pub label: String,
    status: String,
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
    okta_config: &OktaConfig,
) -> Result<HashMap<String, OktaUser>, BiError> {
    let mut users_map = HashMap::new();
    let mut url = format!("{}/api/v1/users?limit=200", okta_config.domain);
    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", okta_config.api_key))
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
    okta_config: &OktaConfig,
) -> Result<Vec<OktaApplication>, BiError> {
    let mut apps = Vec::new();
    let mut url = format!(
        "{}/api/v1/apps?limit=200&filter=status eq \"ACTIVE\"",
        okta_config.domain
    );

    let users_map = fetch_all_okta_users(client, okta_config).await?;

    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", okta_config.api_key))
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
            let users = get_users_assigned_to_app(client, okta_config, &app.id, &users_map).await?;
            app.embedded = Some(OktaEmbeddedUsers { users });
        }

        apps.extend(page_apps);

        if let Some(next) = next_link {
            url = next;
        } else {
            break;
        }
    }

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

async fn get_users_assigned_to_app(
    client: &Client,
    okta_config: &OktaConfig,
    app_id: &str,
    users_map: &HashMap<String, OktaUser>,
) -> Result<Vec<OktaUser>, BiError> {
    let mut all_users = Vec::new();
    let mut url = format!(
        "{}/api/v1/apps/{}/users?limit=450",
        okta_config.domain, app_id
    );

    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", okta_config.api_key))
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
                    user.profile.email.clone_from(&full_user.profile.email);
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

pub async fn create_sso_config_and_assign_identities(
    api_client: &ApiClient,
    okta_application: &OktaApplication,
) -> Result<sso_configs::SsoConfigBookmark, BiError> {
    let login_link = okta_application
        ._links
        .app_links
        .first()
        .ok_or_else(|| BiError::StringError("No app_link present".to_string()))?;
    let logo = okta_application
        ._links
        .logo
        .first()
        .cloned()
        .unwrap_or(Logo {
            name: "default".to_string(),
            href: "https://static.byndid.com/logos/beyondidentity.png".to_string(),
            r#type: "image/png".to_string(),
        });
    let sso_config = sso_configs::create_sso_config(
        &api_client,
        okta_application.label.clone(),
        login_link.href.clone(),
        Some(logo.href),
    )
    .await?;

    let beyond_identity_identities = IdentitiesService::new()
        .build()
        .await
        .list_identities(None, None)
        .await?
        .identities;
    let filtered_identities = filter_identities(
        &okta_application.embedded.as_ref().unwrap().users,
        &beyond_identity_identities,
    );

    sso_configs::assign_identities_to_sso_config(&api_client, &sso_config, &filtered_identities)
        .await?;

    Ok(sso_config)
}
