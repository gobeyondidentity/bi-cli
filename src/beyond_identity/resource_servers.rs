use crate::beyond_identity::api_token::get_beyond_identity_api_token;
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceServer {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub display_name: String,
    pub identifier: String,
}

pub async fn fetch_beyond_identity_resource_servers(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<ResourceServer>, BiError> {
    let mut resource_servers = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/resource-servers?page_size=100",
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
        let page_resource_servers: Vec<ResourceServer> =
            serde_json::from_value(response_json["resource_servers"].clone())?;

        resource_servers.extend(page_resource_servers);

        if let Some(next_page_token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            url = format!(
                "{}/v1/tenants/{}/realms/{}/resource-servers?page_size=200&page_token={}",
                config.beyond_identity_api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                next_page_token
            );
        } else {
            break;
        }
    }

    Ok(resource_servers)
}
