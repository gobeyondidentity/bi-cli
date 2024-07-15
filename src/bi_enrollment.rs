use crate::bi_api_token::get_beyond_identity_api_token;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Write};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityResponse {
    pub identities: Vec<Identity>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub realm_id: String,
    pub tenant_id: String,
    pub display_name: String,
    pub create_time: String,
    pub update_time: String,
    pub traits: IdentityTraits,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityTraits {
    pub username: String,
    pub primary_email_address: String,
}

pub async fn get_all_identities(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<Vec<Identity>, BiError> {
    let mut all_identities = Vec::new();
    let mut next_page_token: Option<String> = None;
    let bi_api_token = get_beyond_identity_api_token(client, config, tenant_config).await?;

    loop {
        let url = match next_page_token {
            Some(ref token) => format!(
                "{}/v1/tenants/{}/realms/{}/identities?page_token={}",
                config.beyond_identity_api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id,
                token
            ),
            None => format!(
                "{}/v1/tenants/{}/realms/{}/identities",
                config.beyond_identity_api_base_url,
                tenant_config.tenant_id,
                tenant_config.realm_id
            ),
        };

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", bi_api_token))
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

pub fn select_identities(identities: &[Identity]) -> Vec<Identity> {
    println!("Select identities to send enrollment email (comma separated indices or 'all' for all identities):");

    for (index, identity) in identities.iter().enumerate() {
        println!(
            "{}: {} - {}",
            index, identity.id, identity.traits.primary_email_address
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
    pub magic_link: MagicLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagicLink {}

pub async fn send_enrollment_email(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    identity: &Identity,
) -> Result<EnrollmentJobResponse, BiError> {
    let bi_api_token = get_beyond_identity_api_token(client, config, tenant_config).await?;
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/identities/{}/enrollment-jobs",
        config.beyond_identity_api_base_url,
        tenant_config.tenant_id,
        tenant_config.realm_id,
        identity.id
    );

    let payload = json!({
        "job": {
            "delivery_method": "EMAIL",
            "delivery_details": {
                "template": "secure_workforce_credential_binding_with_platform_authenticator_download_link"
            },
            "verification_details": {
                "magic_link": {}
            }
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", bi_api_token))
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
