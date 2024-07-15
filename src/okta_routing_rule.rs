use crate::error::BiError;
use crate::okta_identity_provider::OktaIdpResponse;
use crate::{config::Config, tenant::TenantConfig};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRule {
    pub id: String,
    pub name: String,
    pub priority: i32,
    pub r#type: String,
    pub conditions: OktaRoutingRuleConditions,
    pub actions: OktaRoutingRuleActions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRuleConditions {
    pub user_identifier: OktaRoutingRuleUserIdentifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRuleUserIdentifier {
    pub attribute: String,
    pub r#type: String,
    pub patterns: Vec<OktaRoutingRulePattern>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRulePattern {
    pub match_type: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRuleExpression {
    pub value: String,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRuleActions {
    pub idp: OktaRoutingRuleIdp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRuleIdp {
    pub providers: Vec<OktaRoutingRuleIdpProvider>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaRoutingRuleIdpProvider {
    pub id: String,
    pub type_: String,
}

pub async fn create_okta_routing_rule(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    okta_idp_config: &OktaIdpResponse,
) -> Result<OktaRoutingRule, BiError> {
    let okta_domain = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();
    let attribute_name = config.okta_registration_sync_attribute.clone();
    let attribute_value = "true";
    let okta_policy = get_first_idp_discovery_policy(client, config).await?;

    let url = format!("{}/api/v1/policies/{}/rules", okta_domain, okta_policy.id);

    let payload = json!({
        "type": "IDP_DISCOVERY",
        "name": format!("Route to Beyond Identity ({})", tenant_config.tenant_id),
        "priority": 1,
        "conditions": {
            "userIdentifier": {
                "type": "ATTRIBUTE",
                "attribute": attribute_name,
                "patterns": [
                    {
                        "matchType": "EQUALS",
                        "value": attribute_value
                    }
                ]
            }
        },
        "actions": {
            "idp": {
                "providers": [
                    {
                        "id": okta_idp_config.id,
                        "type": "OIDC"
                    }
                ]
            }
        }
    });

    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let routing_rule: OktaRoutingRule = serde_json::from_str(&response_text)?;

    let serialized = serde_json::to_string_pretty(&routing_rule)?;

    let config_path = config.file_paths.okta_routing_rule.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    Ok(routing_rule)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaPolicyResponse {
    pub id: String,
    pub status: String,
    pub name: String,
    pub created: String,
    pub last_updated: String,
    pub priority: i32,
    pub r#type: String,
}

async fn get_first_idp_discovery_policy(
    client: &Client,
    config: &Config,
) -> Result<OktaPolicyResponse, BiError> {
    let okta_domain = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();

    let url = format!("{}/api/v1/policies?type=IDP_DISCOVERY", okta_domain);

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let policies: Vec<OktaPolicyResponse> = serde_json::from_str(&response_text)?;
    if let Some(policy) = policies.into_iter().find(|p| p.r#type == "IDP_DISCOVERY") {
        Ok(policy)
    } else {
        Err(BiError::RequestError(
            status,
            "No IDP_DISCOVERY policy found".to_string(),
        ))
    }
}

pub async fn load_okta_routing_rule(config: &Config) -> Result<OktaRoutingRule, BiError> {
    let config_path = config.file_paths.okta_routing_rule.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let okta_routing_rule: OktaRoutingRule =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(okta_routing_rule)
}
