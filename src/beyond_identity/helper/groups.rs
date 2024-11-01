use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::beyond_identity::api::common::service::GroupsService;
use crate::beyond_identity::api::common::service::IdentitiesService;
use crate::beyond_identity::api::groups::api::GroupsApi;
use crate::beyond_identity::api::groups::types::DeleteMembersRequest;
use crate::beyond_identity::api::identities::api::IdentitiesApi;
use crate::beyond_identity::api::identities::types::Identity;
use crate::beyond_identity::helper::enrollment::get_credentials_for_identity;
use crate::beyond_identity::helper::enrollment::Credential;
use crate::common::error::BiError;

pub async fn delete_group_memberships(identity_id: &str) -> Result<(), BiError> {
    let groups = IdentitiesService::new()
        .build()
        .await
        .list_groups(identity_id, None)
        .await?
        .groups;

    for group in groups {
        GroupsService::new()
            .build()
            .await
            .delete_members(
                &group.id,
                &DeleteMembersRequest {
                    identity_ids: vec![identity_id.to_string()],
                },
            )
            .await?;
    }

    Ok(())
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
