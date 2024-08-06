use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OneLoginUser {
    id: i64,
    email: String,
    username: String,
    #[serde(rename = "firstname")]
    given_name: String,
    #[serde(rename = "lastname")]
    family_name: String,
    distinguished_name: Option<String>,
    userprincipalname: Option<String>,
    samaccountname: Option<String>,
    member_of: Option<String>,
    #[serde(rename = "custom_attributes")]
    custom_attributes: CustomAttributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomAttributes {
    adpid: Option<String>,
    concur: Option<String>,
    email_verified: Option<String>,
    stf_role: Option<String>,
    us_associate: Option<String>,
}

// Map of <OneLoginUserID, BeyondIdentityIdentityID>
pub type IdentityMapping = HashMap<String, String>;

pub async fn onelogin_create_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_token: &str,
    bi_token: &str,
) -> Result<IdentityMapping, BiError> {
    // Fetch all users (paginated).
    // TODO: Remove the limits when I'm ready to fully run this.
    let list_users_url = format!("{}/api/2/users?limit=3", config.onelogin_base_url);
    let mut identities_mapping = IdentityMapping::new();

    if let Ok(data) = fs::read_to_string(&config.file_paths.onelogin_identities_mapping) {
        if let Ok(existing_mapping) = serde_json::from_str::<IdentityMapping>(&data) {
            identities_mapping = existing_mapping;
        }
    }

    let mut new_identities_created = 0;

    loop {
        let response = client
            .get(&list_users_url)
            .header("Authorization", format!("Bearer {}", onelogin_token))
            .send()
            .await?;

        let users: Vec<serde_json::Value> = response.json().await?;

        // Create identities in Beyond Identity.
        for user in users {
            if let Some(user_id) = user["id"].as_i64() {
                // If service account, skip.
                if user["username"].as_str().unwrap_or("").is_empty() {
                    log::info!(
                        "Skipping user with empty username - Service Account, ID: {}",
                        user_id
                    );
                    continue;
                }

                // If identity already created, skip.
                if identities_mapping.contains_key(&user_id.to_string()) {
                    log::info!("User already mapped, skipping. ID: {}", user_id);
                    continue;
                }

                // if the id is already in the map, then skip as well
                let user_url = format!("{}/api/2/users/{}", config.onelogin_base_url, user_id);
                let user_response = client
                    .get(&user_url)
                    .header("Authorization", format!("Bearer {}", onelogin_token))
                    .send()
                    .await?;

                let onelogin_user: OneLoginUser = user_response.json().await?;

                match create_identity(client, config, tenant_config, bi_token, &onelogin_user).await
                {
                    Ok(bi_identity_id) => {
                        identities_mapping.insert(user_id.to_string(), bi_identity_id);
                        new_identities_created += 1;
                    }
                    Err(e) => {
                        log::error!("Failed to create identity for user {}: {:?}", user, e);
                    }
                }
            }
        }

        // let after_cursor = response
        //     .headers()
        //     .get("After-Cursor")
        //     .and_then(|v| v.to_str().ok())
        //     .map(String::from);

        // if let Some(cursor) = after_cursor {
        //     url = format!(
        //         "{}/api/2/users?after_cursor={}",
        //         config.onelogin_base_url, cursor
        //     );
        // } else {
        //     break;
        // }
        break;
    }
    let serialized = serde_json::to_string_pretty(&identities_mapping)?;
    let config_path = config.file_paths.onelogin_identities_mapping.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    log::info!("New identities created: {}", new_identities_created);

    Ok(identities_mapping)
}

// Creates a Beyond Identity based on the OneLogin user.
// Returns the ID of the identity created in Beyond Identity.
// NOTE: If it is a 409 conflict error, we also return the ID from the resource info.
pub async fn create_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    user: &OneLoginUser,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let payload = json!({
        "identity": {
            "display_name": format!("{} {}", user.given_name, user.family_name),
            "traits": {
                "type": "traits_v0",
                "primary_email_address": user.email,
                "username": user.username,
                "given_name": user.given_name,
                "family_name": user.family_name
            }
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", bi_token))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::info!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if status.is_success() {
        let bi_identity: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bi_identity["id"].as_str().unwrap().to_string())
    } else if status.as_u16() == 409 {
        let error_response: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(details) = error_response["details"].as_array() {
            if let Some(detail) = details.first() {
                if let Some(id) = detail["id"].as_str() {
                    log::warn!(
                        "Identity already exists for user {}, returning identity ID: {}",
                        user.id,
                        id.to_string()
                    );
                    return Ok(id.to_string());
                }
            }
        }
        Err(BiError::RequestError(
            status,
            serde_json::to_string(&error_response)?,
        ))
    } else {
        Err(BiError::RequestError(status, response_text))
    }
}
