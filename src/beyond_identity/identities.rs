use crate::beyond_identity::api_token::get_beyond_identity_api_token;
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn delete_all_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<(), BiError> {
    let identities = fetch_beyond_identity_identities(client, config, tenant_config).await?;

    for identity in identities {
        let url = format!(
            "{}/v1/tenants/{}/realms/{}/identities{}",
            config.beyond_identity_api_base_url,
            tenant_config.tenant_id,
            tenant_config.realm_id,
            identity.id,
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

        println!("Deleted {} ({})", identity.id, identity.display_name);
    }

    Ok(())
}

pub async fn fetch_beyond_identity_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let mut identities = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities?page_size=200",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
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
                config.beyond_identity_api_base_url,
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
