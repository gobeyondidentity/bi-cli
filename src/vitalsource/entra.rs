use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct EntraUser {
    #[serde(rename = "accountEnabled")]
    account_enabled: bool,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "employeeId")]
    employee_id: Option<String>,
    #[serde(rename = "givenName")]
    given_name: Option<String>,
    id: String,
    mail: Option<String>,
    #[serde(rename = "onPremisesExtensionAttributes")]
    attributes: OnPremisesExtensionAttributes,
    #[serde(rename = "onPremisesSamAccountName")]
    sam_account_name: Option<String>,
    surname: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OnPremisesExtensionAttributes {
    #[serde(rename = "extensionAttribute1")]
    extension_attribute1: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EntraResponse {
    value: Vec<EntraUser>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
}

pub async fn vitalsource_entra_sync(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    entra_token: &str,
    bi_token: &str,
) -> Result<(), BiError> {
    let mut url = format!(
        "https://graph.microsoft.com/v1.0/users?\
    $select=id,displayName,givenName,surname,userPrincipalName,mail,\
    onPremisesSamAccountName,onPremisesExtensionAttributes,employeeID,accountEnabled&\
    $filter=endswith(userPrincipalName,'@vitalsource.com')&\
    $orderby=userPrincipalName&\
    $count=true"
    );

    loop {
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", entra_token))
            .header("ConsistencyLevel", "eventual")
            .send()
            .await?;

        let entra_response: EntraResponse = response.json().await?;

        for user in entra_response.value {
            let username = &user.user_principal_name;
            match bi_get_identity_by_username(client, config, tenant_config, bi_token, username)
                .await
            {
                Ok(identity_id) => {
                    if identity_id != "" {
                        log::info!(
                            "Found user {:?} with ID: {}, attempting to PATCH",
                            username,
                            identity_id
                        );
                        // If user is found, update the existing identity.
                        match bi_update_identity(
                            client,
                            config,
                            tenant_config,
                            bi_token,
                            &user,
                            &identity_id,
                        )
                        .await
                        {
                            Ok(identity_id) => {
                                log::info!(
                                    "Successfully updated identity with ID: {}",
                                    identity_id
                                );
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to update identity for user {:?} id {:?}: {:?}",
                                    username,
                                    identity_id,
                                    e
                                );
                            }
                        }
                    } else {
                        // If user is enabled and not already in BI directory, create the identity.
                        // This is the logic for new users that are onboarded.
                        if user.account_enabled {
                            log::info!(
                            "Did not find a user in BI with username: {:?}, attempting to CREATE a new identity",
                            username,
                        );
                            match bi_create_identity(client, config, tenant_config, bi_token, &user)
                                .await
                            {
                                Ok(identity_id) => {
                                    log::info!(
                                        "Successfully created identity with ID: {}",
                                        identity_id
                                    );
                                }
                                Err(e) => {
                                    log::error!(
                                        "Failed to create identity for user {:?}: {:?}",
                                        username,
                                        e
                                    );
                                }
                            }
                        } else {
                            log::info!(
                            "Did not find a user in BI with usernamee {:?}, but they are inactive, skipping",
                            username,
                        );
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to get identity by username {:?}: {:?}",
                        &user.user_principal_name,
                        e
                    );
                }
            }
        }

        if let Some(next_link) = entra_response.next_link {
            url = next_link;
        } else {
            break;
        }

        // break;
    }

    Ok(())
}

async fn bi_get_identity_by_username(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    username: &str,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?filter=traits.username eq \"{}\"",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        username
    );

    log::info!("Get identity by username: {}", url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let response: serde_json::Value = serde_json::from_str(&response_text)?;

    if response["total_size"] == 0 {
        return Ok(String::new());
    }

    let identities = response["identities"].as_array().unwrap();
    let first_identity = identities[0].as_object().unwrap();
    let id = first_identity["id"].as_str().unwrap();

    Ok(id.to_string())
}

// Creates a Beyond Identity based on the Entra user.
// Returns the ID of the identity created in Beyond Identity.
async fn bi_create_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    user: &EntraUser,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let mut traits = json!({
        "type": "traits_v0",
        "primary_email_address": user.mail,
        "username": user.user_principal_name,
        "given_name": user.given_name,
        "family_name": user.surname
    });

    let upn = &user.user_principal_name;
    if !upn.is_empty() {
        traits["userprinciplename"] = json!(upn);
    }
    if let Some(sam) = &user.sam_account_name {
        if !sam.is_empty() {
            traits["samaccountname"] = json!(sam);
        }
    }
    if let Some(adp) = &user.employee_id {
        if !adp.is_empty() {
            traits["adp_id"] = json!(adp);
        }
    }
    if let Some(concur) = &user.attributes.extension_attribute1 {
        if !concur.is_empty() {
            traits["concur_id"] = json!(concur);
        }
    }

    let payload = json!({
        "identity": {
            "display_name": format!("{} {}", user.given_name.as_deref().unwrap_or(""), user.surname.as_deref().unwrap_or("")),
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

    log::info!(
        "CreateUser with status {} URL: {} and response: {}",
        status,
        url,
        response_text
    );

    if status.is_success() {
        let bi_identity: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bi_identity["id"].as_str().unwrap().to_string())
    } else {
        Err(BiError::RequestError(status, response_text))
    }
}

// // Creates a Beyond Identity based on the OneLogin user.
// // Returns the ID of the identity created in Beyond Identity.
async fn bi_update_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    user: &EntraUser,
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
        "primary_email_address": user.mail,
        "given_name": user.given_name,
        "family_name": user.surname
    });

    let upn = &user.user_principal_name;
    if !upn.is_empty() {
        traits["userprinciplename"] = json!(upn);
    }
    if let Some(sam) = &user.sam_account_name {
        if !sam.is_empty() {
            traits["samaccountname"] = json!(sam);
        }
    }
    if let Some(adp) = &user.employee_id {
        if !adp.is_empty() {
            traits["adp_id"] = json!(adp);
        }
    }
    if let Some(concur) = &user.attributes.extension_attribute1 {
        if !concur.is_empty() {
            traits["concur_id"] = json!(concur);
        }
    }

    let payload = json!({
        "identity": {
            "display_name": format!("{} {}", user.given_name.as_deref().unwrap_or(""), user.surname.as_deref().unwrap_or("")),
            "status": if user.account_enabled { "active" } else { "suspended" },
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

    log::info!(
        "UpdateUser with status: {} URL: {} and response: {}",
        status,
        url,
        response_text
    );

    if status.is_success() {
        let bi_identity: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bi_identity["id"].as_str().unwrap().to_string())
    } else {
        Err(BiError::RequestError(status, response_text))
    }
}
