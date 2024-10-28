use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::helper::enrollment::get_credentials_for_identity;
use crate::beyond_identity::helper::{enrollment::Credential, identities::Identity};
use crate::common::error::BiError;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub display_name: String,
}

pub async fn delete_group_memberships(
    api_client: &ApiClient,
    identity_id: &str,
) -> Result<(), BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let groups = fetch_group_memberships(api_client, identity_id).await?;

    for group in groups {
        let url = format!(
            "{}/v1/tenants/{}/realms/{}/groups/{}:deleteMembers",
            realm.api_base_url, tenant.id, realm.id, group.id,
        );

        let response = api_client
            .client
            .post(&url)
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

        println!(
            "Unassigned identity {} from group {}",
            identity_id, group.id
        );
    }
    Ok(())
}

pub async fn fetch_group_memberships(
    api_client: &ApiClient,
    identity_id: &str,
) -> Result<Vec<Group>, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let mut groups = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}:listGroups",
        realm.api_base_url, tenant.id, realm.id, identity_id
    );

    loop {
        let response = api_client.client.get(&url).send().await?;

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
                realm.api_base_url, tenant.id, realm.id, identity_id, next_page_token
            );
        } else {
            break;
        }
    }

    Ok(groups)
}

pub async fn fetch_all_groups(api_client: &ApiClient) -> Result<Vec<Group>, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let mut groups = Vec::new();
    let mut url = format!(
        "{}/v1/tenants/{}/realms/{}/groups",
        realm.api_base_url, tenant.id, realm.id
    );

    loop {
        let response = api_client.client.get(&url).send().await?;

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
                "{}/v1/tenants/{}/realms/{}/groups?page_size=200&page_token={}",
                realm.api_base_url, tenant.id, realm.id, next_page_token
            );
        } else {
            break;
        }
    }

    Ok(groups)
}

pub async fn get_identities_from_group(
    api_client: &ApiClient,
    group_id: &str,
) -> Result<Vec<Identity>, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let mut identities = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let url = match &next_page_token {
            Some(token) => format!(
                "{}/v1/tenants/{}/realms/{}/groups/{}:listMembers?page_token={}",
                realm.api_base_url, tenant.id, realm.id, group_id, token
            ),
            None => format!(
                "{}/v1/tenants/{}/realms/{}/groups/{}:listMembers",
                realm.api_base_url, tenant.id, realm.id, group_id
            ),
        };

        let response = api_client.client.get(&url).send().await?;

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

        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let page_identities: Vec<Identity> =
            serde_json::from_value(response_json["identities"].clone())?;

        identities.extend(page_identities);

        if let Some(token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            next_page_token = Some(token.to_string());
        } else {
            break;
        }
    }

    Ok(identities)
}

pub async fn get_unenrolled_identities_from_group(
    api_client: &ApiClient,
    group_id: &str,
) -> Result<Vec<Identity>, BiError> {
    let (tenant, realm) = match api_client.db.get_default_tenant_and_realm().await? {
        Some((t, r)) => (t, r),
        None => {
            return Err(BiError::StringError(
                "No default tenant/realm set".to_string(),
            ))
        }
    };

    let mut identities = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let url = match &next_page_token {
            Some(token) => format!(
                "{}/v1/tenants/{}/realms/{}/groups/{}:listMembers?page_token={}",
                realm.api_base_url, tenant.id, realm.id, group_id, token
            ),
            None => format!(
                "{}/v1/tenants/{}/realms/{}/groups/{}:listMembers",
                realm.api_base_url, tenant.id, realm.id, group_id
            ),
        };

        let response = api_client.client.get(&url).send().await?;

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

        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let page_identities: Vec<Identity> =
            serde_json::from_value(response_json["identities"].clone())?;

        let mut unenrolled_identities = Vec::new();

        for i in page_identities {
            let credentials = get_credentials_for_identity(api_client, &i.id)
                .await
                .expect("Failed to fetch credentials");
            let enrolled = credentials
                .into_iter()
                .filter(|cred| cred.realm_id == realm.id && cred.tenant_id == tenant.id)
                .collect::<Vec<Credential>>();
            if enrolled.is_empty() {
                unenrolled_identities.push(i);
            }
        }

        identities.extend(unenrolled_identities);

        if let Some(token) = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str())
        {
            next_page_token = Some(token.to_string());
        } else {
            break;
        }
    }

    Ok(identities)
}
