use crate::common::error::BiError;
use crate::setup::tenants::tenant::TenantConfig;

use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub resource_server_id: String,
    pub display_name: String,
}

pub async fn delete_role_memberships(
    client: &Client,
    tenant_config: &TenantConfig,
    identity_id: &str,
    resource_server_id: &str,
) -> Result<(), BiError> {
    let roles =
        fetch_role_memberships(client, tenant_config, identity_id, resource_server_id).await?;

    for role in roles {
        let url = format!(
            "{}/v1/tenants/{}/realms/{}/resource-servers/{}/roles/{}:deleteMembers",
            tenant_config.api_base_url,
            tenant_config.tenant_id,
            tenant_config.realm_id,
            role.resource_server_id,
            role.id,
        );

        let response = client
            .post(&url)
            .json(&serde_json::json!({
                "group_ids": [],
                "identity_ids": [identity_id]
            }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            log::debug!("{} response status: {}", url, status);
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        println!("Unassigned identity {} from role {}", identity_id, role.id);
    }
    Ok(())
}

pub async fn fetch_role_memberships(
    client: &Client,
    tenant_config: &TenantConfig,
    identity_id: &str,
    resource_server_id: &str,
) -> Result<Vec<Role>, BiError> {
    let mut roles = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}:listRoles?resource_server_id={}",
        tenant_config.api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        identity_id,
        resource_server_id,
    );

    loop {
        let response = client.get(&url).send().await?;

        let status = response.status();
        log::debug!("{} response status: {}", url, status);
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let response_text = response.text().await?;
        log::debug!("{} response text: {}", url, response_text);
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let page_roles: Vec<Role> = serde_json::from_value(response_json["roles"].clone())?;

        roles.extend(page_roles);

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/identities/{}:listRoles?page_size=200&page_token={}",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                identity_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(roles)
}

pub async fn fetch_beyond_identity_roles(
    client: &Client,
    tenant_config: &TenantConfig,
    resource_server_id: &str,
) -> Result<Vec<Role>, BiError> {
    let mut roles = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/resource-servers/{}/roles",
        tenant_config.api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        resource_server_id,
    );

    loop {
        let response = client.get(&url).send().await?;

        let status = response.status();
        log::debug!("{} response status: {}", url, status);
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let response_text = response.text().await?;
        log::debug!("{} response text: {}", url, response_text);
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let page_roles: Vec<Role> = serde_json::from_value(response_json["roles"].clone())?;

        roles.extend(page_roles);

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/resource-servers/{}/roles?page_size=200&page_token={}",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                resource_server_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(roles)
}
