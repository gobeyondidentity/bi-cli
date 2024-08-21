use super::identities::IdentityMapping;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Deserialize)]
struct OneLoginRole {
    id: i64,
    name: String,
    users: Vec<i64>,
}

// Map of <OneLoginRoleID, BeyondIdentityGroupID>
pub type GroupMapping = HashMap<String, String>;

// Map of <BeyondIdentityGroupID, [BeyondIdentityIdentityIDs]>
pub type GroupAssignmentMapping = HashMap<String, Vec<String>>;

pub async fn vitalsource_create_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_token: &str,
    bi_token: &str,
) -> Result<GroupMapping, BiError> {
    let mut group_mapping = GroupMapping::new();

    if let Ok(data) = fs::read_to_string(&config.file_paths.vitalsource_group_mapping) {
        if let Ok(existing_mapping) = serde_json::from_str::<GroupMapping>(&data) {
            group_mapping = existing_mapping;
        }
    }

    let mut list_roles_url = format!("{}/api/2/roles", config.onelogin_base_url);
    let mut new_groups_created = 0;

    loop {
        let response = client
            .get(&list_roles_url)
            .header("Authorization", format!("Bearer {}", onelogin_token))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        let after_cursor = response
            .headers()
            .get("After-Cursor")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let roles: Vec<serde_json::Value> = response.json().await?;

        for role in roles {
            if let Some(role_id) = role["id"].as_i64() {
                if group_mapping.contains_key(&role_id.to_string()) {
                    log::info!("Role already mapped, skipping. ID: {}", role_id);
                    continue;
                }

                let onelogin_role = OneLoginRole {
                    id: role_id,
                    name: role["name"].as_str().unwrap_or("").to_string(),
                    users: Vec::new(), // not needed in this step
                };

                match bi_create_group(client, config, tenant_config, &bi_token, &onelogin_role)
                    .await
                {
                    Ok(bi_group_id) => {
                        group_mapping.insert(role_id.to_string(), bi_group_id);
                        new_groups_created += 1;
                    }
                    Err(e) => {
                        log::error!("Failed to create group for role {}: {:?}", role, e);
                    }
                }
            }
        }

        if let Some(cursor) = after_cursor {
            list_roles_url = format!("{}/api/2/roles?cursor={}", config.onelogin_base_url, cursor);
        } else {
            break;
        }
    }

    let serialized = serde_json::to_string_pretty(&group_mapping)?;
    let config_path = config.file_paths.vitalsource_group_mapping.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    log::info!("New groups created: {}", new_groups_created);

    Ok(group_mapping)
}

// Creates a Beyond Identity based on the OneLogin Role.
// Returns the ID of the group created in Beyond Identity.
async fn bi_create_group(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    role: &OneLoginRole,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/groups",
        config.beyond_identity_api_base_url, tenant_config.tenant_id, tenant_config.realm_id
    );
    let payload = json!({
        "group": {
            "display_name": role.name,
            "description": format!("Group created from OneLogin Role ID {}, Role Name {}", role.id, role.name)
        }
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::info!("{} URL: {} and response: {}", status, url, response_text);

    if !status.is_success() {
        Err(BiError::RequestError(status, response_text))
    } else {
        let bi_group: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(bi_group["id"].as_str().unwrap().to_string())
    }
}

// Assigns members to groups based on the OneLogin role mapping.
pub async fn vitalsource_assign_identities_to_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_access_token: &str,
    bi_token: &str,
    groups_mapping: GroupMapping,
    identities_mapping: IdentityMapping,
) -> Result<(), BiError> {
    let mut group_assignments = GroupAssignmentMapping::new();

    for (onelogin_role_id, bi_group_id) in groups_mapping {
        let url = format!(
            "{}/api/2/roles/{}",
            config.onelogin_base_url, onelogin_role_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", onelogin_access_token))
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }
        let role: OneLoginRole = response.json().await?;

        let bi_identity_ids: Vec<String> = role
            .users
            .iter()
            .filter_map(|user_id| identities_mapping.get(&user_id.to_string()))
            .cloned()
            .collect();

        group_assignments.insert(bi_group_id.clone(), bi_identity_ids.clone());

        if !bi_identity_ids.is_empty() {
            add_members_to_group(
                client,
                config,
                tenant_config,
                bi_token,
                &bi_group_id,
                bi_identity_ids,
            )
            .await?;
        }
    }

    let serialized = serde_json::to_string_pretty(&group_assignments)?;
    let config_path = config
        .file_paths
        .vitalsource_group_assignment_mapping
        .clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    log::info!("OneLogin Migration Group Assignments Completed!");

    Ok(())
}

// Adds members to a Beyond Identity group.
async fn add_members_to_group(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_token: &str,
    group_id: &str,
    identity_ids: Vec<String>,
) -> Result<(), BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/groups/{}:addMembers",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        group_id
    );

    let payload = json!({
        "identity_ids": identity_ids
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bi_token))
        .json(&payload)
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
