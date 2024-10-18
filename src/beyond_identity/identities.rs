use crate::beyond_identity::enrollment::{get_credentials_for_identity, Credential};
use crate::beyond_identity::roles::{delete_role_memberships, fetch_role_memberships};
use crate::beyond_identity::tenant::TenantConfig;
use crate::beyond_identity::{
    api_token::get_beyond_identity_api_token, groups::delete_group_memberships,
};
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};

use super::resource_servers::fetch_beyond_identity_resource_servers;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub display_name: String,
    pub create_time: String,
    pub update_time: String,
    pub traits: IdentityTraits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityTraits {
    pub username: String,
    pub primary_email_address: Option<String>,
}

pub async fn fetch_beyond_identity_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let mut identities = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?page_size=200",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
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
                tenant_config.api_base_url,
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

pub async fn delete_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, identity_id,
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
        log::debug!("{} response status: {}", url, status);
        let error_text = response.text().await?;
        return Err(BiError::RequestError(status, error_text));
    }

    Ok(())
}

pub async fn delete_all_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<(), BiError> {
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?page_size=200",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let resource_servers = fetch_beyond_identity_resource_servers(&client, &config, &tenant_config)
        .await
        .expect("Failed to fetch resource servers");

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

        for identity in &page_identities {
            delete_group_memberships(&client, &config, &tenant_config, &identity.id)
                .await
                .expect("Failed to delete role memberships");
            for rs in &resource_servers {
                delete_role_memberships(&client, &config, &tenant_config, &identity.id, &rs.id)
                    .await
                    .expect("Failed to delete role memberships");
            }
            delete_identity(&client, &config, &tenant_config, &identity.id)
                .await
                .expect("Failed to delete identity");

            println!("Deleted Identity: {}", identity.id);
        }

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/identities?page_size=200&page_token={}",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(())
}

pub async fn delete_unenrolled_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<(), BiError> {
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?page_size=200",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let resource_servers = fetch_beyond_identity_resource_servers(&client, &config, &tenant_config)
        .await
        .expect("Failed to fetch resource servers");

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

        for identity in &page_identities {
            let credentials =
                get_credentials_for_identity(client, config, tenant_config, &identity.id)
                    .await
                    .expect("Failed to fetch credentials");
            let enrolled = credentials
                .into_iter()
                .filter(|cred| {
                    cred.realm_id == tenant_config.realm_id
                        && cred.tenant_id == tenant_config.tenant_id
                })
                .collect::<Vec<Credential>>();
            if enrolled.is_empty() {
                delete_group_memberships(&client, &config, &tenant_config, &identity.id)
                    .await
                    .expect("Failed to delete role memberships");
                for rs in &resource_servers {
                    delete_role_memberships(&client, &config, &tenant_config, &identity.id, &rs.id)
                        .await
                        .expect("Failed to delete role memberships");
                }
                delete_identity(&client, &config, &tenant_config, &identity.id)
                    .await
                    .expect("Failed to delete identity");

                println!("Deleted Identity: {}", identity.id);
            }
        }

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/identities?page_size=200&page_token={}",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(())
}

pub async fn delete_norole_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<(), BiError> {
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?page_size=200",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );

    let resource_servers = fetch_beyond_identity_resource_servers(&client, &config, &tenant_config)
        .await
        .expect("Failed to fetch resource servers");

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

        for identity in &page_identities {
            let mut has_role = false;
            for resource_server in &resource_servers {
                let roles = fetch_role_memberships(
                    client,
                    config,
                    tenant_config,
                    &identity.id,
                    &resource_server.id,
                )
                .await?;

                has_role |= !roles.is_empty();
            }

            if !has_role {
                delete_group_memberships(&client, &config, &tenant_config, &identity.id)
                    .await
                    .expect("Failed to delete role memberships");
                for rs in &resource_servers {
                    delete_role_memberships(&client, &config, &tenant_config, &identity.id, &rs.id)
                        .await
                        .expect("Failed to delete role memberships");
                }
                delete_identity(&client, &config, &tenant_config, &identity.id)
                    .await
                    .expect("Failed to delete identity");

                println!("Deleted Identity: {}", identity.id);
            }
        }

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/identities?page_size=200&page_token={}",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(())
}
