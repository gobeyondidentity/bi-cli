use crate::beyond_identity::helper::identities::Identity;
use crate::beyond_identity::helper::tenant::TenantConfig;
use crate::common::error::BiError;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, Write};

use super::groups::Group;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CredentialResponse {
    pub credentials: Vec<Credential>,
    pub total_size: i32,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub tenant_id: String,
    pub realm_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SsoConfigIdpResponse {
    /// The ID of this SSO config.
    pub id: String,
    /// SSO config name for display purposes.
    pub display_name: String,
    /// The payload of the SSO config.
    pub payload: IdpPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdpPayload {
    #[serde(rename = "GenericOidcIdp")]
    pub generic_oidc_idp: GenericOidcIdp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenericOidcIdp {
    pub identity_provider_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityResponse {
    pub identities: Vec<Identity>,
    pub next_page_token: Option<String>,
}

pub async fn get_all_identities(
    client: &Client,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let mut all_identities = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let url = match next_page_token {
            Some(ref token) => format!(
                "{}/v1/tenants/{}/realms/{}/identities?page_token={}",
                tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, token
            ),
            None => format!(
                "{}/v1/tenants/{}/realms/{}/identities",
                tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id
            ),
        };

        let response = client.get(&url).send().await?;

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

        let identity_response: IdentityResponse = serde_json::from_str(&response_text)?;

        all_identities.extend(identity_response.identities);

        if let Some(token) = identity_response.next_page_token {
            next_page_token = Some(token);
        } else {
            break;
        }
    }

    Ok(all_identities)
}

pub async fn get_credentials_for_identity(
    client: &Client,
    tenant_config: &TenantConfig,
    identity_id: &str,
) -> Result<Vec<Credential>, BiError> {
    let mut all_credentials = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let url = match next_page_token {
            Some(ref token) => format!(
                "{}/v1/tenants/{}/realms/{}/identities/{}/credentials?page_token={}",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                identity_id,
                token
            ),
            None => format!(
                "{}/v1/tenants/{}/realms/{}/identities/{}/credentials",
                tenant_config.api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                identity_id,
            ),
        };

        let response = client.get(&url).send().await?;

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

        let credential_response: CredentialResponse = serde_json::from_str(&response_text)?;

        all_credentials.extend(credential_response.credentials);

        if let Some(token) = credential_response.next_page_token {
            next_page_token = Some(token);
        } else {
            break;
        }
    }

    Ok(all_credentials)
}

pub async fn get_unenrolled_identities(
    client: &Client,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let identities = get_all_identities(client, tenant_config)
        .await
        .expect("Failed to fetch identities");

    let mut unenrolled_identities = Vec::new();

    for i in identities {
        let credentials = get_credentials_for_identity(client, tenant_config, &i.id)
            .await
            .expect("Failed to fetch credentials");
        let enrolled = credentials
            .into_iter()
            .filter(|cred| {
                cred.realm_id == tenant_config.realm_id && cred.tenant_id == tenant_config.tenant_id
            })
            .collect::<Vec<Credential>>();
        if enrolled.is_empty() {
            unenrolled_identities.push(i);
        }
    }

    Ok(unenrolled_identities)
}

// We expose the sso config to Ike but not the actual idp application. This will get that application_id for us.
pub async fn get_idp_application_for_sso_config(
    client: &Client,
    tenant_config: &TenantConfig,
    sso_config_id: String,
) -> Result<SsoConfigIdpResponse, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/sso-configs/{}",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, sso_config_id
    );

    let response = client.get(&url).send().await?;

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

    let sso_config: SsoConfigIdpResponse = serde_json::from_str(&response_text)?;

    Ok(sso_config)
}

pub fn select_identities(identities: &[Identity]) -> Vec<Identity> {
    println!("Select identities (comma separated indices or 'all' for all identities):");

    for (index, identity) in identities.iter().enumerate() {
        println!(
            "{}: {} - {}",
            index,
            identity.id,
            identity
                .traits
                .primary_email_address
                .as_deref()
                .unwrap_or("<no email provided>")
        );
    }

    print!("Your selection: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "all" {
        return identities.to_vec();
    }

    let indices: Vec<usize> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    indices.into_iter().map(|i| identities[i].clone()).collect()
}

pub fn select_group(groups: &[Group]) -> Group {
    println!("Select a group by entering its index:");

    for (index, group) in groups.iter().enumerate() {
        println!("{}: {} - {}", index, group.id, group.display_name);
    }

    print!("Your selection: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Parse the input as a usize and ensure it's a valid index.
    match input.parse::<usize>() {
        Ok(index) if index < groups.len() => groups[index].clone(),
        _ => {
            println!("Invalid selection. Please try again.");
            select_group(groups) // Retry if the selection is invalid.
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrollmentJobResponse {
    pub enrollment_job: EnrollmentJob,
    pub enrollment_job_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrollmentJob {
    pub tenant_id: String,
    pub realm_id: String,
    pub identity_id: String,
    pub delivery_method: String,
    pub delivery_details: DeliveryDetails,
    pub verification_details: VerificationDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDetails {
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationDetails {
    pub magic_link: Option<MagicLink>,
    pub idp_authorization: Option<IdpAuthorization>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdpAuthorization {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagicLink {}

pub async fn get_send_email_payload(
    client: &Client,
    tenant_config: &TenantConfig,
) -> Result<Value, BiError> {
    let template = "secure_workforce_credential_binding_with_platform_authenticator_download_link";

    let mut payload = json!({
        "job": {
            "delivery_method": "EMAIL",
            "delivery_details": {
                "template": template
            },
            "verification_details": {
                "magic_link": {}
            }
        }
    });

    println!("Enter enrollment method: magic_link or idp");

    print!("Your selection: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "idp" {
        println!("Please enter the delegate identity provider ID:");

        print!("Your selection: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Ike only has acces to the sso_config_id but we need the identity_provider_id
        let sso_config =
            get_idp_application_for_sso_config(client, tenant_config, input.to_string())
                .await
                .expect("Failed to load get identity provider sso config.");

        payload = json!({
            "job": {
                "delivery_method": "EMAIL",
                "delivery_details": {
                    "template": template
                },
                "verification_details": {
                    "idp_authorization": {
                        "identity_provider_id": sso_config.payload.generic_oidc_idp.identity_provider_id,
                        "identity_provider_display_name": sso_config.display_name
                    },
                }
            }
        });
    };

    Ok(payload)
}

pub async fn send_enrollment_email(
    client: &Client,
    tenant_config: &TenantConfig,
    identity: &Identity,
    payload: Value,
) -> Result<EnrollmentJobResponse, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}/enrollment-jobs",
        tenant_config.api_base_url, tenant_config.tenant_id, tenant_config.realm_id, identity.id
    );

    let response = client
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

    let job: EnrollmentJobResponse = serde_json::from_str(&response_text)?;
    Ok(job)
}
