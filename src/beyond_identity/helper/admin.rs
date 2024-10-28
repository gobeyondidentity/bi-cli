use super::identities::{fetch_beyond_identity_identities, Identity};
use super::resource_servers::fetch_beyond_identity_resource_servers;
use super::roles::{fetch_beyond_identity_roles, fetch_role_memberships};

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::common::error::BiError;

pub async fn create_admin_account(
    api_client: &ApiClient,
    email: String,
) -> Result<Identity, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let resource_servers = fetch_beyond_identity_resource_servers(api_client).await?;

    let bi_management_api_rs = resource_servers
        .iter()
        .find(|rs| rs.identifier == "beyondidentity")
        .ok_or(BiError::StringError(
            "No beyondidentity resource server found in your realm".to_string(),
        ))?;

    let roles = fetch_beyond_identity_roles(api_client, &bi_management_api_rs.id).await?;

    let super_admin_role = roles.iter().find(|role| role.display_name == "Super Administrators").ok_or(BiError::StringError("No Super Administrators role found in your Beyond Identity Management API resource server".to_string()))?;

    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities",
        realm.api_base_url, tenant.id, realm.id,
    );

    let response = api_client
        .client
        .post(&url)
        .json(&serde_json::json!({
            "identity": {
                "display_name": email,
                "status": "active",
                "traits": {
                    "type": "traits_v0",
                    "username": email,
                    "primary_email_address": email,
                }

            }
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        log::debug!("{} response status: {}", url, status);
        let error_text = response.text().await?;
        return Err(BiError::RequestError(status, error_text));
    }

    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
    let identity: Identity = serde_json::from_value(response_json.clone())?;

    let url = format!(
        "{}/v1/tenants/{}/realms/{}/resource-servers/{}/roles/{}:addMembers",
        realm.api_base_url, tenant.id, realm.id, bi_management_api_rs.id, super_admin_role.id,
    );

    let response = api_client
        .client
        .post(&url)
        .json(&serde_json::json!({
            "group_ids": [],
            "identity_ids": [&identity.id]
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        log::debug!("{} response status: {}", url, status);
        let error_text = response.text().await?;
        return Err(BiError::RequestError(status, error_text));
    }

    Ok(identity)
}

pub async fn get_identities_without_role(api_client: &ApiClient) -> Result<Vec<Identity>, BiError> {
    let identities = fetch_beyond_identity_identities(api_client).await?;
    let resource_servers = fetch_beyond_identity_resource_servers(api_client).await?;

    let mut identities_without_roles = vec![];
    for identity in &identities {
        let mut has_role = false;
        for resource_server in &resource_servers {
            let roles =
                fetch_role_memberships(api_client, &identity.id, &resource_server.id).await?;

            has_role |= !roles.is_empty();
        }

        if !has_role {
            identities_without_roles.push(identity.clone());
        }
    }

    Ok(identities_without_roles)
}
