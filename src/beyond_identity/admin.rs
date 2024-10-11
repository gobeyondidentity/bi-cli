use crate::beyond_identity::api_token::get_beyond_identity_api_token;
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest::Client;

use super::identities::Identity;
use super::resource_servers::fetch_beyond_identity_resource_servers;
use super::roles::fetch_beyond_identity_roles;

pub async fn create_admin_account(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Identity, BiError> {
    let email = &config.admin_primary_email_address;
    let resource_servers =
        fetch_beyond_identity_resource_servers(client, config, tenant_config).await?;

    let bi_management_api_rs = resource_servers
        .iter()
        .find(|rs| rs.identifier == "beyondidentity")
        .ok_or(BiError::StringError(
            "No beyondidentity resource server found in your realm".to_string(),
        ))?;

    let roles =
        fetch_beyond_identity_roles(client, config, tenant_config, &bi_management_api_rs.id)
            .await?;

    let super_admin_role = roles.iter().find(|role| role.display_name == "Super Administrators").ok_or(BiError::StringError("No Super Administrators role found in your Beyond Identity Management API resource server".to_string()))?;

    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id,
    );

    let response = client
        .post(&url)
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                get_beyond_identity_api_token(client, config, tenant_config).await?
            ),
        )
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
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        bi_management_api_rs.id,
        super_admin_role.id,
    );

    let response = client
        .post(&url)
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                get_beyond_identity_api_token(client, config, tenant_config).await?
            ),
        )
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
