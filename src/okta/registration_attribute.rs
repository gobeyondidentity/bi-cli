use crate::common::config::Config;
use crate::common::error::BiError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaUserSchema {
    pub id: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub last_updated: String,
    pub created: String,
    pub definitions: Definitions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definitions {
    pub custom: Custom,
    pub base: Base,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub id: String,
    pub r#type: String,
    pub properties: CustomProperties,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomProperties {
    pub byndid_registered: ByndidRegistered,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByndidRegistered {
    pub title: String,
    pub description: String,
    pub r#type: String,
    pub default: bool,
    pub mutability: String,
    pub scope: String,
    pub permissions: Vec<Permission>,
    pub master: Master,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub principal: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    pub id: String,
    pub r#type: String,
    pub properties: BaseProperties,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseProperties {
    pub login: BaseProperty,
    pub first_name: BaseProperty,
    pub last_name: BaseProperty,
    pub email: BaseProperty,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseProperty {
    pub title: String,
    pub r#type: String,
    pub required: bool,
    pub mutability: String,
    pub scope: String,
    pub permissions: Vec<Permission>,
    pub master: Master,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub self_link: Link,
    pub type_link: Link,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub rel: String,
    pub href: String,
    pub method: String,
}

pub async fn create_custom_attribute(
    client: &Client,
    config: &Config,
) -> Result<OktaUserSchema, BiError> {
    let okta_domain = &config.okta_domain;
    let okta_api_key = &config.okta_api_key;

    let url = format!("{}/api/v1/meta/schemas/user/default", okta_domain);

    let payload = json!({
        "definitions": {
            "custom": {
                "properties": {
                    config.okta_registration_sync_attribute.clone(): {
                        "type": "boolean",
                        "title": "Beyond Identity Registration Attribute",
                        "description": "A registration attribute used for routing rules when using Beyond Identity as a Delegate IdP",
                        "required": false,
                        "default": false,
                    }
                }
            }
        }
    });

    let response = client
        .post(&url)
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

    let user_schema: OktaUserSchema = serde_json::from_str(&response_text)?;

    let serialized = serde_json::to_string_pretty(&user_schema)?;

    let config_path = config.file_paths.okta_custom_attribute.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    Ok(user_schema)
}

pub async fn load_custom_attribute(config: &Config) -> Result<OktaUserSchema, BiError> {
    let config_path = config.file_paths.okta_custom_attribute.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let okta_user_schema: OktaUserSchema =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(okta_user_schema)
}
