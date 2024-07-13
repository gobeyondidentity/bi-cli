use crate::bi_api_token::get_beyond_identity_api_token;
use crate::config::Config;
use crate::error::BiError;
use crate::tenant::TenantConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct ResourceServer {
    id: String,
    #[allow(dead_code)]
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct ResourceServerResponse {
    resource_servers: Vec<ResourceServer>,
}

async fn get_first_resource_server(
    client: &Client,
    tenant_id: &str,
    realm_id: &str,
    beyond_identity_api_token: &str,
    base_url: &str,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/resource-servers",
        base_url, tenant_id, realm_id
    );

    let response = client
        .get(&url)
        .header(
            "Authorization",
            format!("Bearer {}", beyond_identity_api_token),
        )
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(
            reqwest::StatusCode::BAD_REQUEST,
            response_text,
        ));
    }

    let resource_server_response: ResourceServerResponse = serde_json::from_str(&response_text)?;
    if let Some(first_resource_server) = resource_server_response.resource_servers.first() {
        Ok(first_resource_server.id.clone())
    } else {
        Err(BiError::RequestError(
            reqwest::StatusCode::NOT_FOUND,
            "No resource servers found".to_string(),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OktaRegistrationPayload {
    domain: String,
    okta_token: String,
    attribute_name: String,
    is_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OktaRegistrationResponse {
    domain: String,
    okta_token: String,
    attribute_name: String,
    is_enabled: bool,
}

async fn create_okta_registration(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<OktaRegistrationResponse, BiError> {
    // Only one Okta registration can exist at a time
    delete_okta_registration(client, config, tenant_config).await?;

    let domain = config.okta_domain.clone();
    let okta_token = config.okta_api_key.clone();
    let attribute_name = config.okta_registration_sync_attribute.clone();
    let beyond_identity_api_base_url = config.beyond_identity_api_base_url.clone();
    let beyond_identity_api_token =
        get_beyond_identity_api_token(client, config, tenant_config).await?;
    let tenant_id = &tenant_config.tenant_id;
    let realm_id = &tenant_config.realm_id;

    let payload = OktaRegistrationPayload {
        domain,
        okta_token,
        attribute_name,
        is_enabled: true,
    };

    let response = client
        .post(format!(
            "{}/v1/tenants/{}/realms/{}/okta-registration",
            beyond_identity_api_base_url, tenant_id, realm_id
        ))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", beyond_identity_api_token),
        )
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let registration_response: OktaRegistrationResponse = serde_json::from_str(&response_text)?;
    Ok(registration_response)
}

async fn delete_okta_registration(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<(), BiError> {
    let beyond_identity_api_base_url = config.beyond_identity_api_base_url.clone();
    let beyond_identity_api_token =
        get_beyond_identity_api_token(client, config, tenant_config).await?;
    let tenant_id = &tenant_config.tenant_id;
    let realm_id = &tenant_config.realm_id;

    let response = client
        .delete(format!(
            "{}/v1/tenants/{}/realms/{}/okta-registration",
            beyond_identity_api_base_url, tenant_id, realm_id
        ))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", beyond_identity_api_token),
        )
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeyondIdentityAppResponse {
    id: String,
    realm_id: String,
    tenant_id: String,
    display_name: String,
    protocol_config: ProtocolConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProtocolConfig {
    client_id: String,
    client_secret: String,
    token_endpoint_auth_method: String,
}

async fn create_scim_app(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<BeyondIdentityAppResponse, BiError> {
    let beyond_identity_api_base_url = config.beyond_identity_api_base_url.clone();
    let beyond_identity_api_token =
        get_beyond_identity_api_token(&client, &config, &tenant_config).await?;
    let tenant_id = tenant_config.tenant_id.clone();
    let realm_id = tenant_config.realm_id.clone();

    // Fetch the first resource server
    let resource_server_id = get_first_resource_server(
        client,
        &tenant_id,
        &realm_id,
        &beyond_identity_api_token,
        &beyond_identity_api_base_url,
    )
    .await?;

    let payload = json!({
        "application": {
            "display_name": "SCIM 2.0 Application",
            "resource_server_id": resource_server_id,
            "protocol_config": {
                "type": "oauth2",
                "allowed_scopes": ["scim:all"],
                "confidentiality": "confidential",
                "grant_type": ["client_credentials"],
                "redirect_uris": [],
                "token_configuration": {
                    "expires_after": 60 * 24 * 90,
                },
            },
            "classification": "scim_with_okta_registration"
        }
    });

    let response = client
        .post(format!(
            "{}/v1/tenants/{}/realms/{}/applications",
            beyond_identity_api_base_url, tenant_id, realm_id
        ))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", beyond_identity_api_token),
        )
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(
            reqwest::StatusCode::BAD_REQUEST,
            response_text,
        ));
    }

    let app_response: BeyondIdentityAppResponse = serde_json::from_str(&response_text)?;
    Ok(app_response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTokenResponse {
    access_token: String,
}

pub async fn generate_scim_app_token(
    client: &Client,
    config: &Config,
    scim_response: &BeyondIdentityAppResponse,
) -> Result<String, BiError> {
    let url = format!(
        "{}/v1/tenants/{}/realms/{}/applications/{}/token",
        config.beyond_identity_auth_base_url,
        scim_response.tenant_id,
        scim_response.realm_id,
        scim_response.id
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(
            scim_response.protocol_config.client_id.clone(),
            Some(scim_response.protocol_config.client_secret.clone()),
        )
        .form(&[
            ("grant_type", "client_credentials"),
            ("scope", "scim:all"),
            ("lifetime", "31536000"), // 1 year in seconds
        ])
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let token_response: ApiTokenResponse = serde_json::from_str(&response_text)?;
    Ok(token_response.access_token)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiScimConfig {
    scim_application_config: BeyondIdentityAppResponse,
    pub oauth_bearer_token: String,
}

async fn load_or_create_beyond_identity_scim_with_okta_registration(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> BiScimConfig {
    let config_path = config.file_paths.bi_scim_app_config.clone();
    if Path::new(&config_path).exists() {
        let data = fs::read_to_string(config_path).expect("Unable to read file");
        let json = serde_json::from_str(&data).expect("JSON was not well-formatted");
        log::debug!(
            "Scim config loaded from file: {}",
            serde_json::to_string_pretty(&json).unwrap()
        );
        return json;
    } else {
        log::debug!("Scim config not found in file. Creating new one.");
        create_okta_registration(client, config, tenant_config)
            .await
            .expect("Failed to create okta registration");
        let response = create_scim_app(client, config, tenant_config)
            .await
            .expect("Failed to create scim application");
        let oauth_bearer_token = generate_scim_app_token(client, config, &response)
            .await
            .expect("Failed to generate a scim app token");
        let bi_scim_config = BiScimConfig {
            scim_application_config: response.clone(),
            oauth_bearer_token: oauth_bearer_token.clone(),
        };
        let serialized = serde_json::to_string_pretty(&bi_scim_config)
            .expect("Failed to serialize tenant response");
        fs::write(config_path, serialized).expect("Unable to write file");

        bi_scim_config
    }
}

pub async fn load_beyond_identity_scim_app(config: &Config) -> Result<BiScimConfig, BiError> {
    let config_path = config.file_paths.bi_scim_app_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let bi_scim_config: BiScimConfig =
        serde_json::from_str(&data).map_err(|err| BiError::SerdeError(err))?;
    Ok(bi_scim_config)
}

pub async fn create_beyond_identity_scim_app(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
) -> Result<BiScimConfig, BiError> {
    create_okta_registration(client, config, tenant_config)
        .await
        .expect("Failed to create okta registration");
    let response = create_scim_app(client, config, tenant_config)
        .await
        .expect("Failed to create scim application");
    let oauth_bearer_token = generate_scim_app_token(client, config, &response)
        .await
        .expect("Failed to generate a scim app token");
    let bi_scim_config = BiScimConfig {
        scim_application_config: response.clone(),
        oauth_bearer_token: oauth_bearer_token.clone(),
    };
    let serialized =
        serde_json::to_string_pretty(&bi_scim_config).expect("Failed to serialize tenant response");
    fs::write(config.file_paths.bi_scim_app_config.clone(), serialized)
        .expect("Unable to write file");

    Ok(bi_scim_config)
}
