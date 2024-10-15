use crate::beyond_identity::api_token::get_beyond_identity_api_token;
use crate::beyond_identity::identities::{fetch_beyond_identity_identities, Identity};
use crate::beyond_identity::resource_servers::fetch_beyond_identity_resource_servers;
use crate::beyond_identity::roles::{fetch_beyond_identity_roles, fetch_role_memberships};
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest_middleware::ClientWithMiddleware as Client;

pub async fn create_admin_account(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    email: String,
) -> Result<Identity, BiError> {
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
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id,
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
        tenant_config.api_base_url,
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

pub async fn get_identities_without_role(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let identities = fetch_beyond_identity_identities(client, config, tenant_config).await?;
    let resource_servers =
        fetch_beyond_identity_resource_servers(client, config, tenant_config).await?;

    let mut identities_without_roles = vec![];
    for identity in &identities {
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
            identities_without_roles.push(identity.clone());
        }
    }

    Ok(identities_without_roles)
}
