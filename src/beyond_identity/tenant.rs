use crate::common::config::Config;
use crate::common::error::BiError;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::fs;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantConfig {
    pub tenant_id: String,
    pub realm_id: String,
    pub application_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub open_id_configuration_url: String,
    pub auth_base_url: String,
    pub api_base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // The issuer field in the JWT
}

pub async fn load_tenant(config: &Config) -> Result<TenantConfig, BiError> {
    let config_path = config.file_paths.tenant_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let tenant_config: TenantConfig = serde_json::from_str(&data).map_err(BiError::SerdeError)?;
    Ok(tenant_config)
}

pub async fn provision_tenant(
    client: &Client,
    config: &Config,
    token: &str,
) -> Result<TenantConfig, BiError> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.insecure_disable_signature_validation();
    validation.validate_aud = false;

    // Decode the JWT (without signature verification)
    let decoded_token = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret("".as_ref()), // A dummy secret since we are disabling signature verification
        &validation,
    ) {
        Ok(token) => token,
        Err(err) => {
            return Err(BiError::StringError(format!(
                "Failed to decode JWT: {:?}",
                err
            )));
        }
    };

    let claims = decoded_token.claims;

    // Extract the issuer (iss) from the claims
    let issuer_url = claims.iss;

    // Parse the URL and extract tenant_id, realm_id, application_id
    let parsed_url = Url::parse(&issuer_url).map_err(BiError::InvalidUrl)?;

    let segments: Vec<&str> = parsed_url
        .path_segments()
        .map_or(vec![], |segments| segments.collect());

    let tenant_id = segments
        .get(2)
        .ok_or(BiError::StringError("Invalid tenant ID".to_string()))?
        .to_string();
    let realm_id = segments
        .get(4)
        .ok_or(BiError::StringError("Invalid realm ID".to_string()))?
        .to_string();
    let application_id = segments
        .get(6)
        .ok_or(BiError::StringError("Invalid application ID".to_string()))?
        .to_string();

    // Extract base URLs from the issuer URL
    let host = parsed_url
        .host_str()
        .ok_or(BiError::StringError("Invalid URL host".to_string()))?;

    let auth_base_url = format!("https://{}", host);
    let api_base_url = auth_base_url.replace("auth", "api");

    let management_api_application =
        get_management_api_application(client, &api_base_url, &tenant_id, &realm_id, token).await?;

    let client_id = management_api_application
        .protocol_config
        .client_id
        .expect("Failed to get client id of management API application");
    let client_secret = management_api_application
        .protocol_config
        .client_secret
        .expect("Failed to get client secret of management API application");

    // Create the tenant configuration
    let tenant_config = TenantConfig {
        tenant_id,
        realm_id,
        application_id,
        client_id,
        client_secret,
        open_id_configuration_url: format!("{}/.well-known/openid-configuration", issuer_url),
        api_base_url,
        auth_base_url,
    };

    // Serialize the tenant configuration to JSON
    let serialized = serde_json::to_string_pretty(&tenant_config).map_err(BiError::SerdeError)?;

    // Ensure the configuration directory exists
    let config_path = config.file_paths.tenant_config.clone();
    let config_dir = std::path::Path::new(&config_path)
        .parent()
        .ok_or_else(|| BiError::UnableToWriteFile(config_path.clone()))?;
    fs::create_dir_all(config_dir).map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

    // Write the JSON payload to the specified tenant_config path
    fs::write(&config_path, serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

    println!("Tenant configuration saved to '{}'", config_path);

    Ok(tenant_config)
}

#[derive(Debug, Serialize, Deserialize)]
struct Application {
    id: String,
    realm_id: String,
    tenant_id: String,
    display_name: String,
    is_managed: bool,
    classification: String,
    protocol_config: ProtocolConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProtocolConfig {
    client_id: Option<String>,
    client_secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApplicationsResponse {
    applications: Vec<Application>,
    total_size: u32,
    next_page_token: Option<String>,
}

async fn get_management_api_application(
    client: &Client,
    api_base_url: &str,
    tenant_id: &str,
    realm_id: &str,
    bearer_token: &str,
) -> Result<Application, BiError> {
    let mut page_token: Option<String> = None;

    loop {
        let mut url = format!(
            "{}/v1/tenants/{}/realms/{}/applications",
            api_base_url, tenant_id, realm_id
        );

        // Append page_token if it exists
        if let Some(ref token) = page_token {
            url = format!("{}?page_token={}", url, token);
        }

        // Send the GET request
        let response = client.get(&url).bearer_auth(bearer_token).send().await?;

        // Handle non-200 responses
        if !response.status().is_success() {
            return Err(BiError::RequestError(
                response.status(),
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string()),
            ));
        }

        // Parse the response
        let response_body: ApplicationsResponse = response.json().await?;

        // Search for the application with classification "management_api"
        for app in response_body.applications {
            if app.classification == "management_api" {
                // Return immediately when the "management_api" application is found
                return Ok(app);
            }
        }

        // Break the loop if there are no more pages
        if response_body.next_page_token.is_none() {
            break;
        }

        // Set the next page token for the next request
        page_token = response_body.next_page_token;
    }

    Err(BiError::StringError(
        "Management API application not found".to_string(),
    ))
}
