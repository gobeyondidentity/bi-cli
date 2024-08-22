use super::groups::GroupAssignmentMapping;
use super::identities::IdentityMapping;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use crate::vitalsource::groups::GroupMapping;
use reqwest::Client;
use serde_json::json;
use std::fs;

pub async fn vitalsource_rollback_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
) -> Result<(), BiError> {
    let mut identity_mapping = IdentityMapping::new();
    let mut identities_deleted = 0;

    if let Ok(data) = fs::read_to_string(&config.file_paths.vitalsource_identity_mapping) {
        if let Ok(existing_mapping) = serde_json::from_str::<IdentityMapping>(&data) {
            identity_mapping = existing_mapping;
        }
    }

    for (_, bi_identity_id) in identity_mapping {
        match bi_delete_identity(client, config, tenant_config, bi_token, &bi_identity_id).await {
            Ok(_) => {
                log::info!("Successfully deleted user {}", bi_identity_id);
                identities_deleted += 1;
            }
            Err(e) => log::error!("Failed to delete user {}: {:?}", bi_identity_id, e),
        }
    }

    log::info!(
        "Rolled back Vitalsource identities! Identities deleted {}",
        identities_deleted
    );

    Ok(())
}

pub async fn bi_delete_identity(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    identity_id: &str,
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        identity_id
    );

    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::info!("{} URL: {} and response: {}", status, url, response_text);

    if !status.is_success() {
        Err(BiError::RequestError(status, response_text))
    } else {
        Ok(())
    }
}

pub async fn vitalsource_rollback_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
) -> Result<(), BiError> {
    let mut group_mapping = GroupMapping::new();
    let mut group_assignment_mapping = GroupAssignmentMapping::new();
    let mut groups_deleted = 0;

    if let Ok(data) = fs::read_to_string(&config.file_paths.vitalsource_group_assignment_mapping) {
        if let Ok(existing_group_assignment_mapping) =
            serde_json::from_str::<GroupAssignmentMapping>(&data)
        {
            group_assignment_mapping = existing_group_assignment_mapping;
        }
    }

    if let Ok(data) = fs::read_to_string(&config.file_paths.vitalsource_group_mapping) {
        if let Ok(existing_group_mapping) = serde_json::from_str::<GroupMapping>(&data) {
            group_mapping = existing_group_mapping;
        }
    }

    for (_, bi_group_id) in group_mapping {
        if let Some(group_members) = group_assignment_mapping.get(&bi_group_id) {
            if !group_members.is_empty() {
                match bi_delete_members_from_group(
                    client,
                    config,
                    tenant_config,
                    bi_token,
                    &bi_group_id,
                    &group_members,
                )
                .await
                {
                    Ok(_) => log::info!("Successfully removed members from group {}", bi_group_id),
                    Err(e) => log::error!(
                        "Failed to remove members from group {}: {:?}",
                        bi_group_id,
                        e
                    ),
                }
            }
        }
        match bi_delete_group(client, config, tenant_config, bi_token, &bi_group_id).await {
            Ok(_) => {
                log::info!("Successfully deleted group {}", bi_group_id);
                groups_deleted += 1;
            }
            Err(e) => log::error!("Failed to delete group {}: {:?}", bi_group_id, e),
        }
    }

    log::info!(
        "Rolled back Vitalsource groups! Groups deleted {}",
        groups_deleted
    );

    Ok(())
}

pub async fn bi_delete_group(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    group_id: &str,
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/groups/{}",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        group_id
    );

    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::info!("{} URL: {} and response: {}", status, url, response_text);

    if !status.is_success() {
        Err(BiError::RequestError(status, response_text))
    } else {
        Ok(())
    }
}

async fn bi_delete_members_from_group(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    group_id: &str,
    member_ids: &[String],
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/groups/{}:deleteMembers",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        group_id
    );

    let payload = json!({
        "identity_ids": member_ids
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(BiError::RequestError(
            response.status(),
            response.text().await?,
        ))
    }
}
