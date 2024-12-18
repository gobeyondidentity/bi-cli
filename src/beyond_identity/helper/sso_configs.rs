use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::identities::types::Identity;
use crate::common::error::BiError;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSsoConfig {
    id: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsoConfigBookmark {
    id: String,
    display_name: String,
    is_migrated: bool,
    payload: SsoConfigPayloadBookmark,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsoConfigPayloadBookmark {
    #[serde(rename = "Bookmark")]
    bookmark: Bookmark,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    login_link: String,
    icon: String,
    is_tile_visible: bool,
    application_tile_id: String,
}

pub async fn create_sso_config(
    api_client: &ApiClient,
    name: String,
    login_link: String,
    icon_url: Option<String>,
) -> Result<SsoConfigBookmark, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs",
        realm.api_base_url, tenant.id, realm.id
    );

    let name = sanitize_label(&name);

    let payload = json!({
        "sso_config": {
            "display_name": name,
            "is_migrated": true,
            "payload": {
                "type": "bookmark",
                "login_link": login_link,
                "icon": icon_url,
                "is_tile_visible": true
            }
        }
    });

    let response = api_client
        .client
        .post(&url)
        .header("Content-Type", "application/json")
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

    let sso_config: SsoConfigBookmark = serde_json::from_str(&response_text)?;
    Ok(sso_config)
}

fn sanitize_label(label: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z\s]").unwrap();
    let sanitized_label: String = re.replace_all(label, "").to_string();
    let trimmed_label = sanitized_label.trim();
    if trimmed_label.len() > 60 {
        trimmed_label[..60].to_string()
    } else {
        trimmed_label.to_string()
    }
}

pub async fn assign_identities_to_sso_config(
    api_client: &ApiClient,
    sso_config: &SsoConfigBookmark,
    identities: &[Identity],
) -> Result<(), BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs/{}:addIdentities",
        realm.api_base_url, tenant.id, realm.id, sso_config.id
    );

    let identity_ids: Vec<String> = identities
        .iter()
        .map(|identity| identity.id.clone())
        .collect();
    let payload = json!({
        "identity_ids": identity_ids,
    });

    let response = api_client
        .client
        .post(&url)
        .header("Content-Type", "application/json")
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

    Ok(())
}
