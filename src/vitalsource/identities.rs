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

pub async fn vitalsource_create_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_token: &str,
    bi_token: &str,
) -> Result<IdentityMapping, BiError> {
    let mut identity_mapping = IdentityMapping::new();

    if let Ok(data) = fs::read_to_string(&config.file_paths.vitalsource_identity_mapping) {
        if let Ok(existing_mapping) = serde_json::from_str::<IdentityMapping>(&data) {
            identity_mapping = existing_mapping;
        }
    }

    // TODO: Remove the limits when I'm ready to fully run this.
    let mut list_users_url = format!("{}/api/2/users?limit=3", config.onelogin_base_url);
    let mut new_identities_created = 0;

    loop {
        let response = client
            .get(&list_users_url)
            .header("Authorization", format!("Bearer {}", onelogin_token))
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
                if identity_mapping.contains_key(&user_id.to_string()) {
                    log::info!("User already mapped, skipping. ID: {}", user_id);
                    continue;
                }

                let user_url = format!("{}/api/2/users/{}", config.onelogin_base_url, user_id);
                let user_response = client
                    .get(&user_url)
                    .header("Authorization", format!("Bearer {}", onelogin_token))
                    .send()
                    .await?;

                let onelogin_user: OneLoginUser = user_response.json().await?;
                match bi_create_identity(client, config, tenant_config, bi_token, &onelogin_user)
                    .await
                {
                    Ok(bi_identity_id) => {
                        identity_mapping.insert(user_id.to_string(), bi_identity_id);
                        new_identities_created += 1;
                    }
                    Err(e) => {
                        log::error!("Failed to create identity for user {}: {:?}", user, e);
                    }
                }
            }
        }

        // TODO: when this is ready, uncomment
        // if let Some(cursor) = after_cursor {
        //     list_users_url = format!("{}/api/2/users?cursor={}", config.onelogin_base_url, cursor);
        // } else {
        //     break;
        // }

        break;
    }

    let serialized = serde_json::to_string_pretty(&identity_mapping)?;
    let config_path = config.file_paths.vitalsource_identity_mapping.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    log::info!("New identities created: {}", new_identities_created);

    Ok(identity_mapping)
}

// Creates a Beyond Identity based on the OneLogin user.
// Returns the ID of the identity created in Beyond Identity.
// NOTE: If it is a 409 conflict error, we also return the ID from the resource info.
pub async fn bi_create_identity(
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

    let mut traits = json!({
        "type": "traits_v0",
        "primary_email_address": user.email,
        "username": user.username,
        "given_name": user.given_name,
        "family_name": user.family_name
    });

    if let Some(dn) = &user.distinguished_name {
        if !dn.is_empty() {
            traits["distinguished_name"] = json!(dn);
        }
    }
    if let Some(upn) = &user.userprincipalname {
        if !upn.is_empty() {
            traits["userprinciplename"] = json!(upn);
        }
    }
    if let Some(sam) = &user.samaccountname {
        if !sam.is_empty() {
            traits["samaccountname"] = json!(sam);
        }
    }
    // TODO: Uncomment this when there are no more restrictions.
    // if let Some(mo) = &user.member_of {
    //     if !mo.is_empty() {
    //         traits["member_of"] = json!(mo);
    //     }
    // }
    if let Some(adp) = &user.custom_attributes.adpid {
        if !adp.is_empty() {
            traits["adp_id"] = json!(adp);
        }
    }
    if let Some(concur) = &user.custom_attributes.concur {
        if !concur.is_empty() {
            traits["concur_id"] = json!(concur);
        }
    }
    if let Some(ev) = &user.custom_attributes.email_verified {
        if !ev.is_empty() {
            traits["email_verified"] = json!(ev);
        }
    }
    if let Some(stf) = &user.custom_attributes.stf_role {
        if !stf.is_empty() {
            traits["stf_role"] = json!(stf);
        }
    }
    if let Some(usa) = &user.custom_attributes.us_associate {
        if !usa.is_empty() {
            traits["us_associate"] = json!(usa);
        }
    }

    let payload = json!({
        "identity": {
            "display_name": format!("{} {}", user.given_name, user.family_name),
            "traits": traits
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

    log::info!("{} URL: {} and response: {}", status, url, response_text);

    if status.is_success() {
        let bi_identity: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bi_identity["id"].as_str().unwrap().to_string())
    } else if status.as_u16() == 409 {
        let error_response: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(details) = error_response["details"].as_array() {
            if let Some(detail) = details.first() {
                if let Some(id) = detail["id"].as_str() {
                    log::warn!(
                        "Identity already exists in Beyond Identity for OneLogin Username: {}, attempting to PATCH",
                        user.username,
                    );
                    match bi_update_identity(
                        client,
                        config,
                        tenant_config,
                        bi_token,
                        user,
                        &id.to_string(),
                    )
                    .await
                    {
                        Ok(_) => return Ok(id.to_string()),
                        Err(e) => log::error!("Failed to update identity: {:?}", e),
                    }
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

// Creates a Beyond Identity based on the OneLogin user.
// Returns the ID of the identity created in Beyond Identity.
// NOTE: If it is a 409 conflict error, we also return the ID from the resource info.
pub async fn bi_update_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    user: &OneLoginUser,
    identity_id: &str,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        identity_id
    );

    let mut traits = json!({
        "type": "traits_v0",
        "primary_email_address": user.email,
        "username": user.username,
        "given_name": user.given_name,
        "family_name": user.family_name
    });

    if let Some(dn) = &user.distinguished_name {
        if !dn.is_empty() {
            traits["distinguished_name"] = json!(dn);
        }
    }
    if let Some(upn) = &user.userprincipalname {
        if !upn.is_empty() {
            traits["userprinciplename"] = json!(upn);
        }
    }
    if let Some(sam) = &user.samaccountname {
        if !sam.is_empty() {
            traits["samaccountname"] = json!(sam);
        }
    }
    // TODO: Uncomment this when there are no more restrictions.
    // if let Some(mo) = &user.member_of {
    //     if !mo.is_empty() {
    //         traits["member_of"] = json!(mo);
    //     }
    // }
    if let Some(adp) = &user.custom_attributes.adpid {
        if !adp.is_empty() {
            traits["adp_id"] = json!(adp);
        }
    }
    if let Some(concur) = &user.custom_attributes.concur {
        if !concur.is_empty() {
            traits["concur_id"] = json!(concur);
        }
    }
    if let Some(ev) = &user.custom_attributes.email_verified {
        if !ev.is_empty() {
            traits["email_verified"] = json!(ev);
        }
    }
    if let Some(stf) = &user.custom_attributes.stf_role {
        if !stf.is_empty() {
            traits["stf_role"] = json!(stf);
        }
    }
    if let Some(usa) = &user.custom_attributes.us_associate {
        if !usa.is_empty() {
            traits["us_associate"] = json!(usa);
        }
    }

    let payload = json!({
        "identity": {
            "display_name": format!("{} {}", user.given_name, user.family_name),
            "traits": traits
        }
    });

    let response = client
        .patch(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", bi_token))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::info!("{} URL: {} and response: {}", status, url, response_text);

    if status.is_success() {
        let bi_identity: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bi_identity["id"].as_str().unwrap().to_string())
    } else if status.as_u16() == 409 {
        let error_response: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(details) = error_response["details"].as_array() {
            if let Some(detail) = details.first() {
                if let Some(id) = detail["id"].as_str() {
                    log::warn!(
                        "Identity already exists for OneLogin user ID: {}, OneLogin Username: {}, returning identity ID: {}",
                        user.username,
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
