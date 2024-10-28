use crate::{beyond_identity::api::common::api_client::ApiClient, common::error::BiError};

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
    api_client: &ApiClient,
    identity_id: &str,
    resource_server_id: &str,
) -> Result<(), BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let roles = fetch_role_memberships(api_client, identity_id, resource_server_id).await?;

    for role in roles {
        let url = format!(
            "{}/v1/tenants/{}/realms/{}/resource-servers/{}/roles/{}:deleteMembers",
            realm.api_base_url, tenant.id, realm.id, role.resource_server_id, role.id,
        );

        let response = api_client
            .client
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
    api_client: &ApiClient,
    identity_id: &str,
    resource_server_id: &str,
) -> Result<Vec<Role>, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let mut roles = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}:listRoles?resource_server_id={}",
        realm.api_base_url, tenant.id, realm.id, identity_id, resource_server_id,
    );

    loop {
        let response = api_client.client.get(&url).send().await?;

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
                realm.api_base_url, tenant.id, realm.id, identity_id, next_page_token
            );
        } else {
            break;
        }
    }

    Ok(roles)
}

pub async fn fetch_beyond_identity_roles(
    api_client: &ApiClient,
    resource_server_id: &str,
) -> Result<Vec<Role>, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let mut roles = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/resource-servers/{}/roles",
        realm.api_base_url, tenant.id, realm.id, resource_server_id,
    );

    loop {
        let response = api_client.client.get(&url).send().await?;

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
                realm.api_base_url, tenant.id, realm.id, resource_server_id, next_page_token
            );
        } else {
            break;
        }
    }

    Ok(roles)
}
