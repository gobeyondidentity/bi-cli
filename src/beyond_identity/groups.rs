use crate::beyond_identity::api_token::get_beyond_identity_api_token;
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub display_name: String,
}

pub async fn delete_group_memberships(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<(), BiError> {
    let groups = fetch_group_memberships(client, config, tenant_config, identity_id).await?;

    for group in groups {
        let url = format!(
            "{}/v1/tenants/{}/realms/{}/groups/{}:deleteMembers",
            tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, group.id,
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

        println!("Deleted {} from {}", identity_id, group.id);
    }
    Ok(())
}

pub async fn fetch_group_memberships(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<Vec<Group>, BiError> {
    let mut groups = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}:listGroups",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, identity_id
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
        let page_groups: Vec<Group> = serde_json::from_value(response_json["groups"].clone())?;

        groups.extend(page_groups);

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/identities/{}:listGroups?page_size=200&page_token={}",
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

    Ok(groups)
}
