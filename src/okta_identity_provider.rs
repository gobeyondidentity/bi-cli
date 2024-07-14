use crate::bi_external_sso::update_application_redirect_uri;
use crate::bi_external_sso::ExternalSSO;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaIdpResponse {
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub status: String,
    pub created: String,
    pub last_updated: String,
    pub protocol: IdpProtocol,
    pub policy: IdpPolicy,
    #[serde(rename = "_links")]
    pub _links: IdpLinks,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpProtocol {
    pub r#type: String,
    pub endpoints: IdpEndpoints,
    pub scopes: Vec<String>,
    pub issuer: IdpIssuer,
    pub credentials: IdpCredentials,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpEndpoints {
    pub authorization: IdpEndpoint,
    pub token: IdpEndpoint,
    pub userinfo: Option<IdpEndpoint>,
    pub jwks: IdpEndpoint,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpEndpoint {
    pub url: String,
    pub binding: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpIssuer {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpCredentials {
    pub client: IdpClientCredentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdpClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpPolicy {
    pub provisioning: IdpProvisioning,
    pub account_link: IdpAccountLink,
    pub subject: IdpSubject,
    pub max_clock_skew: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpProvisioning {
    pub action: String,
    pub profile_master: bool,
    pub groups: IdpGroupProvisioning,
    pub conditions: IdpProvisioningConditions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpGroupProvisioning {
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpProvisioningConditions {
    pub deprovisioned: IdpProvisioningAction,
    pub suspended: IdpProvisioningAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpProvisioningAction {
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpAccountLink {
    pub filter: Option<String>,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpSubject {
    pub user_name_template: IdpUserNameTemplate,
    pub filter: Option<String>,
    pub match_type: String,
    pub match_attribute: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpUserNameTemplate {
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpLinks {
    pub authorize: IdpLink,
    pub client_redirect_uri: IdpLink,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpLink {
    pub href: String,
    pub hints: Option<IdpLinkHints>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdpLinkHints {
    pub allow: Vec<String>,
}

async fn create_idp(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    external_sso_config: &ExternalSSO,
) -> Result<OktaIdpResponse, BiError> {
    let okta_domain = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();

    let client_id = external_sso_config.protocol_config.client_id.clone();
    let client_secret = external_sso_config.protocol_config.client_secret.clone();
    let issuer = format!(
        "{}/v1/tenants/{}/realms/{}/applications/{}",
        config.beyond_identity_auth_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        external_sso_config.id
    );
    let authorization_endpoint = format!(
        "{}/v1/tenants/{}/realms/{}/applications/{}/authorize",
        config.beyond_identity_auth_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        external_sso_config.id
    );
    let token_endpoint = format!(
        "{}/v1/tenants/{}/realms/{}/applications/{}/token",
        config.beyond_identity_auth_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        external_sso_config.id
    );
    let jwks_endpoint = format!(
        "{}/v1/tenants/{}/realms/{}/applications/{}/.well-known/jwks.json",
        config.beyond_identity_auth_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        external_sso_config.id
    );

    let payload = json!({
        "type": "OIDC",
        "name": "Beyond Identity IDP",
        "protocol": {
            "type": "OIDC",
            "scopes": ["email", "openid"],
            "credentials": {
                "client": {
                    "client_id": client_id,
                    "client_secret": client_secret
                }
            },
            "issuer": {
                "url": issuer
            },
            "endpoints": {
                "authorization": {
                    "binding": "HTTP-REDIRECT",
                    "url": authorization_endpoint
                },
                "token": {
                    "binding": "HTTP-POST",
                    "url": token_endpoint
                },
                "jwks": {
                    "binding": "HTTP-REDIRECT",
                    "url": jwks_endpoint
                }
            }
        },
        "policy": {
            "accountLink": {
                "action": "AUTO",
                "filter": null
            },
            "provisioning": {
                "action": "AUTO",
                "profileMaster": false,
                "conditions": {
                    "deprovisioned": {
                        "action": "NONE"
                    },
                    "suspended": {
                        "action": "NONE"
                    }
                },
                "groups": {
                    "action": "NONE"
                }
            },
            "subject": {
                "userNameTemplate": {
                    "template": "idpuser.email"
                },
                "matchType": "USERNAME"
            },
            "maxClockSkew": 120000
        }
    });

    let response = client
        .post(&format!("{}/api/v1/idps", okta_domain))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let idp_response: OktaIdpResponse = serde_json::from_str(&response_text)?;
    Ok(idp_response)
}

pub async fn load_okta_identity_provider(config: &Config) -> Result<OktaIdpResponse, BiError> {
    let config_path = config.file_paths.okta_identity_provider.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let okta_idp_response: OktaIdpResponse =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(okta_idp_response)
}

pub async fn create_okta_identity_provider(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    external_sso_config: &ExternalSSO,
) -> Result<OktaIdpResponse, BiError> {
    let config_path = config.file_paths.okta_identity_provider.clone();
    let response = create_idp(client, config, tenant_config, external_sso_config).await?;
    update_application_redirect_uri(
        client,
        config,
        tenant_config,
        &external_sso_config.id,
        &response._links.client_redirect_uri.href,
    )
    .await?;
    let serialized = serde_json::to_string_pretty(&response)?;
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;
    Ok(response)
}
