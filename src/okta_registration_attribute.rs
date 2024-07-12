use crate::config::Config;
use crate::error::BiError;
use reqwest::Client;
use serde_json::json;

pub async fn add_custom_attribute_to_okta_user_type(
    client: &Client,
    config: &Config,
) -> Result<(), BiError> {
    let okta_domain = &config.okta_domain;
    let okta_api_key = &config.okta_api_key;

    // Construct the dynamic payload using the json! macro
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
        .post(&format!("{}/api/v1/meta/schemas/user/default", okta_domain))
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

    Ok(())
}
