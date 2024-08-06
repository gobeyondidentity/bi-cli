use crate::bi_api_token::get_beyond_identity_api_token;
use crate::config::Config;
use crate::error::BiError;
use crate::onelogin::token::get_onelogin_access_token;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::{groups::GroupMapping, identities::IdentityMapping};

// Map of <OneLoginApplicationID, BeyondIdentityApplicationID>
type ApplicationMapping = HashMap<String, String>;

pub async fn onelogin_create_applications(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_access_token: &str,
) -> Result<ApplicationMapping, BiError> {
    let mut application_mapping = ApplicationMapping::new();
    Ok(application_mapping)
}

pub async fn onelogin_assign_groups_to_applications(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_access_token: &str,
    applications_mapping: ApplicationMapping,
    group_mapping: GroupMapping,
) -> Result<(), BiError> {
    Ok(())
}

pub async fn onelogin_assign_identities_to_applications(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    onelogin_access_token: &str,
    applications_mapping: ApplicationMapping,
    identities_mapping: IdentityMapping,
) -> Result<(), BiError> {
    Ok(())
}