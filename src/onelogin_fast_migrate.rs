use crate::bi_api_token::get_beyond_identity_api_token;
use crate::bi_enrollment::get_all_identities;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use urlencoding::encode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneLoginApplication {
    id: String,
    display_name: String,
    flavor: String,
    icon_url: String,
    login_link: String,
    is_tile_visible: bool,
    bi_bookmark_app_id: Option<String>,
    onelogin_users: Option<Vec<OneLoginUser>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneLoginUser {
    id: String,
    email: String,
    username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeyondIdentityUser {
    id: String,
    email: String,
    username: String,
}

type OneLoginToBeyondIdentityUserMapping = HashMap<String, String>;

pub async fn load_onelogin_applications(
    config: &Config,
) -> Result<Vec<OneLoginApplication>, BiError> {
    let config_path = config.file_paths.onelogin_applications_mapping.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let onelogin_applications: Vec<OneLoginApplication> =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(onelogin_applications)
}

async fn get_onelogin_access_token(client: &Client, config: &Config) -> Result<String, BiError> {
    println!("fetching OneLogin access token");
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

pub async fn fetch_onelogin_applications(
    client: &Client,
    config: &Config,
) -> Result<Vec<OneLoginApplication>, BiError> {
    println!("Fetching OneLogin Applications.");
    let mut applications = Vec::new();
    let mut url = format!("{}/api/2/apps", config.onelogin_base_url);

    let access_token = get_onelogin_access_token(client, config).await?;
    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let after_cursor = response
            .headers()
            .get("After-Cursor")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let response_json: Vec<serde_json::Value> = response.json().await?;

        for app in response_json {
            let app_id = app["id"].as_i64().expect("App id should be present");
            // Fetch additional details for each application
            let one_login_app =
                fetch_application_details(client, config, &app_id.to_string(), &access_token)
                    .await?;
            applications.push(one_login_app);
        }

        // Check for After-Cursor header
        if let Some(cursor) = after_cursor {
            url = format!("{}/api/2/apps?cursor={}", config.onelogin_base_url, cursor);
        } else {
            break; // No more pages
        }
    }

    let applications_with_users =
        get_users_for_onelogin_applications(client, config, &access_token, applications).await?;

    Ok(applications_with_users)
}

async fn fetch_application_details(
    client: &Client,
    config: &Config,
    app_id: &str,
    access_token: &str,
) -> Result<OneLoginApplication, BiError> {
    let url = format!("{}/api/2/apps/{}", config.onelogin_base_url, app_id);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let app_details: serde_json::Value = response.json().await?;

    Ok(OneLoginApplication {
        id: app_id.to_string(),
        display_name: app_details["name"].as_str().unwrap_or("").to_string(),
        icon_url: app_details["icon_url"].as_str().unwrap_or("").to_string(),
        flavor: app_details["auth_method_description"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        login_link: format!("{}/launch/{}", config.onelogin_base_url, app_id),
        is_tile_visible: app_details["visible"].as_bool().unwrap_or(false),
        bi_bookmark_app_id: Some(String::new()), // Add this field with a default value
        onelogin_users: Some(Vec::new()),
    })
}

async fn get_users_for_onelogin_applications(
    client: &Client,
    config: &Config,
    access_token: &str,
    onelogin_applications: Vec<OneLoginApplication>,
) -> Result<Vec<OneLoginApplication>, BiError> {
    let mut updated_applications = Vec::new();

    for app in onelogin_applications {
        let updated_app =
            get_users_for_onelogin_application(client, config, access_token, app).await?;
        updated_applications.push(updated_app);
    }

    Ok(updated_applications)
}

async fn get_users_for_onelogin_application(
    client: &Client,
    config: &Config,
    access_token: &str,
    mut onelogin_application: OneLoginApplication,
) -> Result<OneLoginApplication, BiError> {
    let mut all_users = Vec::new();
    let mut url = format!(
        "{}/api/2/apps/{}/users",
        config.onelogin_base_url, onelogin_application.id
    );

    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let after_cursor = response
            .headers()
            .get("After-Cursor")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let users: Vec<serde_json::Value> = response.json().await?;

        let onelogin_users: Vec<OneLoginUser> = users
            .into_iter()
            .map(|user| OneLoginUser {
                id: user["id"].as_i64().unwrap_or(0).to_string(),
                email: user["email"].as_str().unwrap_or("").to_string(),
                username: user["username"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        all_users.extend(onelogin_users);

        if let Some(cursor) = after_cursor {
            url = format!("{}&cursor={}", url, cursor);
        } else {
            break;
        }
    }

    onelogin_application.onelogin_users = Some(all_users);

    Ok(onelogin_application)
}

pub async fn match_onelogin_user_to_beyond_identity_user(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<OneLoginToBeyondIdentityUserMapping, BiError> {
    println!("Match OneLogin User To Beyond Identity User.");
    let mut user_mapping = HashMap::new();

    // List all users in BI
    let bi_identities = get_all_identities(client, config, tenant_config).await?;

    // Fetch OneLogin access token
    let onelogin_access_token = get_onelogin_access_token(client, config).await?;

    // Query for each user in OneLogin by email
    for i in bi_identities {
        // println!("Beyond Identity User: {:?}\n", i);

        let email = i.traits.primary_email_address;
        let onelogin_user =
            fetch_onelogin_user_by_email(client, config, &onelogin_access_token, &email).await?;
        // println!("OneLogin User {:?}\n", onelogin_user);
        // Map the OneLogin user to BI identity
        if let Some(onelogin_user) = onelogin_user {
            user_mapping.insert(onelogin_user.id, i.id);
        } else {
            println!("!!!!!Unable to find onelogin user for email == {}\n", email);
        }
    }

    Ok(user_mapping)
}

async fn fetch_onelogin_user_by_email(
    client: &Client,
    config: &Config,
    access_token: &str,
    email: &str,
) -> Result<Option<OneLoginUser>, BiError> {
    let url = format!(
        "{}/api/2/users?email={}",
        config.onelogin_base_url,
        encode(email)
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let users: Vec<serde_json::Value> = response.json().await?;

    if let Some(user) = users.first() {
        Ok(Some(OneLoginUser {
            id: user["id"].as_i64().unwrap_or(0).to_string(),
            email: user["email"].as_str().unwrap_or("").to_string(),
            username: user["username"].as_str().unwrap_or("").to_string(),
        }))
    } else {
        Ok(None)
    }
}

pub async fn create_bookmark_sso_config_and_update_mapping(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_applications: Vec<OneLoginApplication>,
) -> Result<Vec<OneLoginApplication>, BiError> {
    println!("Creating bookmark configs for OneLogin apps");
    let bi_api_token = get_beyond_identity_api_token(client, config, tenant_config).await?;

    let mut updated_applications = Vec::new();

    for app in onelogin_applications {
        println!(
            "Creating bookmark config for OneLogin app: {}",
            app.display_name
        );

        let bookmark_config_id =
            create_bookmark_config(client, config, tenant_config, &bi_api_token, app.clone())
                .await?;

        let mut updated_app = app;
        updated_app.bi_bookmark_app_id = Some(bookmark_config_id);
        updated_applications.push(updated_app);
    }

    Ok(updated_applications)
}

async fn create_bookmark_config(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_api_token: &str,
    onelogin_app: OneLoginApplication,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let payload = json!({
        "sso_config": {
            "display_name": onelogin_app.display_name,
            "payload": {
                "type": "bookmark",
                "login_link": onelogin_app.login_link,
                "icon": onelogin_app.icon_url,
                "is_tile_visible": onelogin_app.is_tile_visible
            }
        }
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bi_api_token))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
    let bookmark_config_id = response_json["id"].as_str().unwrap().to_string();

    Ok(bookmark_config_id)
}

pub async fn assign_users_to_bookmark_apps(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_applications: Vec<OneLoginApplication>,
    user_mapping: OneLoginToBeyondIdentityUserMapping,
) -> Result<(), BiError> {
    println!("Assigning users to bookmark apps");
    let bi_api_token = get_beyond_identity_api_token(client, config, tenant_config).await?;

    for app in onelogin_applications {
        if let Some(bi_bookmark_app_id) = app.bi_bookmark_app_id {
            let url = format!(
                "{}/v1/tenants/{}/realms/{}/sso-configs/{}:addIdentities",
                config.beyond_identity_api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                bi_bookmark_app_id
            );

            let onelogin_users = app.onelogin_users.unwrap_or_default();
            let bi_user_ids: Vec<String> = onelogin_users
                .iter()
                .filter_map(|ol_user| user_mapping.get(&ol_user.id))
                .cloned()
                .collect();

            let payload = json!({
                "identity_ids": bi_user_ids
            });

            let response = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", bi_api_token))
                .json(&payload)
                .send()
                .await?;

            let status = response.status();
            let response_text = response.text().await?;

            if !status.is_success() {
                return Err(BiError::RequestError(status, response_text));
            }

            println!(
                "Assigned {} users to app: {}",
                bi_user_ids.len(),
                app.display_name
            );
        }
    }

    Ok(())
}
