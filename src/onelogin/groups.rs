use crate::bi_api_token::get_beyond_identity_api_token;
use crate::config::Config;
use crate::error::BiError;
use crate::onelogin::token::get_onelogin_access_token;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::identities::IdentityMapping;

// Map of <OneLoginRoleID, BeyondIdentityGroupID>
pub type GroupMapping = HashMap<String, String>;

pub async fn onelogin_create_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_access_token: &str,
) -> Result<GroupMapping, BiError> {
    let mut group_mapping = GroupMapping::new();
    Ok(group_mapping)
}

pub async fn onelogin_assign_identities_to_groups(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_access_token: &str,
    groups_mapping: GroupMapping,
    identities_mapping: IdentityMapping,
) -> Result<(), BiError> {
    Ok(())
}
