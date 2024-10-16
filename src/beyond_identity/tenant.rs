use crate::common::config::Config;
use crate::common::error::BiError;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct TenantKey {
    tenant_id: String,
    realm_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tenants {
    default_tenant_key: Option<TenantKey>,
    tenants: Vec<TenantConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

async fn load_tenants(config: &Config) -> Result<Tenants, BiError> {
    let config_path = config.file_paths.tenants_config.clone();
    let data = fs::read_to_string(&config_path).map_err(|_| {
        BiError::ConfigFileNotFound(format!(
            "Try provisioning a tenant first: {:?}",
            config_path.clone()
        ))
    })?;
    let tenants: Tenants = serde_json::from_str(&data).map_err(BiError::SerdeError)?;
    Ok(tenants)
}

async fn set_default_tenant(
    config: &Config,
    tenant_id: &str,
    realm_id: &str,
) -> Result<(), BiError> {
    let mut tenants = load_tenants(config).await?;

    if tenants
        .tenants
        .iter()
        .any(|t| t.tenant_id == tenant_id && t.realm_id == realm_id)
    {
        tenants.default_tenant_key = Some(TenantKey {
            tenant_id: tenant_id.to_string(),
            realm_id: realm_id.to_string(),
        });

        // Serialize and save back
        let serialized = serde_json::to_string_pretty(&tenants).map_err(BiError::SerdeError)?;
        let config_path = config.file_paths.tenants_config.clone();

        fs::write(&config_path, serialized)
            .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

        Ok(())
    } else {
        Err(BiError::StringError("Tenant not found".to_string()))
    }
}

async fn delete_tenant(config: &Config, tenant_id: &str, realm_id: &str) -> Result<(), BiError> {
    let mut tenants = load_tenants(config).await?;

    let original_length = tenants.tenants.len();
    tenants
        .tenants
        .retain(|t| !(t.tenant_id == tenant_id && t.realm_id == realm_id));

    if tenants.tenants.len() == original_length {
        return Err(BiError::StringError("Tenant not found".to_string()));
    }

    // If the deleted tenant was the default, update default_tenant_key
    if let Some(default_key) = &tenants.default_tenant_key {
        if default_key.tenant_id == tenant_id && default_key.realm_id == realm_id {
            tenants.default_tenant_key = tenants.tenants.first().map(|t| TenantKey {
                tenant_id: t.tenant_id.clone(),
                realm_id: t.realm_id.clone(),
            });
        }
    }

    // Serialize and save back
    let serialized = serde_json::to_string_pretty(&tenants).map_err(BiError::SerdeError)?;
    let config_path = config.file_paths.tenants_config.clone();

    fs::write(&config_path, serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

    Ok(())
}

pub async fn load_tenant(config: &Config) -> Result<TenantConfig, BiError> {
    let tenants = load_tenants(config).await?;
    if let Some(default_key) = tenants.default_tenant_key {
        tenants
            .tenants
            .into_iter()
            .find(|t| t.tenant_id == default_key.tenant_id && t.realm_id == default_key.realm_id)
            .ok_or_else(|| BiError::StringError("Default tenant not found".to_string()))
    } else {
        Err(BiError::StringError("No default tenant set".to_string()))
    }
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
        tenant_id: tenant_id.clone(),
        realm_id: realm_id.clone(),
        application_id,
        client_id,
        client_secret,
        open_id_configuration_url: format!("{}/.well-known/openid-configuration", issuer_url),
        api_base_url,
        auth_base_url,
    };

    // Load existing tenants
    let mut tenants = match load_tenants(config).await {
        Ok(tenants) => tenants,
        Err(_) => Tenants {
            default_tenant_key: None,
            tenants: Vec::new(),
        },
    };

    // Ensure tenant_key is unique
    if tenants
        .tenants
        .iter()
        .any(|t| t.tenant_id == tenant_id && t.realm_id == realm_id)
    {
        return Err(BiError::StringError("Tenant already exists".to_string()));
    }

    // Add the new tenant to the list
    tenants.tenants.push(tenant_config.clone());

    // Set default tenant if none
    if tenants.default_tenant_key.is_none() {
        tenants.default_tenant_key = Some(TenantKey {
            tenant_id,
            realm_id,
        });
    }

    // Serialize the tenants and save back
    let serialized = serde_json::to_string_pretty(&tenants).map_err(BiError::SerdeError)?;
    let config_path = config.file_paths.tenants_config.clone();

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

pub async fn list_tenants_ui(config: &Config) -> Result<(), BiError> {
    // List Tenants
    let tenants = load_tenants(config).await?;
    if tenants.tenants.is_empty() {
        println!("No tenants found.");
    } else {
        println!("List of Tenants:");
        for (index, tenant) in tenants.tenants.iter().enumerate() {
            let default_marker = if Some(TenantKey {
                tenant_id: tenant.tenant_id.clone(),
                realm_id: tenant.realm_id.clone(),
            }) == tenants.default_tenant_key
            {
                "(Default)"
            } else {
                ""
            };
            println!(
                "{}: Tenant ID: {}, Realm ID: {} {}",
                index + 1,
                tenant.tenant_id,
                tenant.realm_id,
                default_marker
            );
        }
    }
    Ok(())
}

pub async fn delete_tenant_ui(config: &Config) -> Result<(), BiError> {
    let tenants = load_tenants(config).await?;
    if tenants.tenants.is_empty() {
        println!("No tenants to delete.");
        return Ok(());
    }

    println!("Select a tenant to delete:");
    for (index, tenant) in tenants.tenants.iter().enumerate() {
        println!(
            "{}: Tenant ID: {}, Realm ID: {}",
            index + 1,
            tenant.tenant_id,
            tenant.realm_id
        );
    }

    print!("Enter the number of the tenant to delete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= tenants.tenants.len() => {
            let tenant = &tenants.tenants[num - 1];
            match delete_tenant(config, &tenant.tenant_id, &tenant.realm_id).await {
                Ok(_) => println!(
                    "Tenant with Tenant ID: {}, Realm ID: {} deleted.",
                    tenant.tenant_id, tenant.realm_id
                ),
                Err(e) => println!("Error deleting tenant: {}", e),
            }
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}

pub async fn set_default_tenant_ui(config: &Config) -> Result<(), BiError> {
    let tenants = load_tenants(config).await?;
    if tenants.tenants.is_empty() {
        println!("No tenants available to set as default.");
        return Ok(());
    }

    println!("Select a tenant to set as default:");
    for (index, tenant) in tenants.tenants.iter().enumerate() {
        println!(
            "{}: Tenant ID: {}, Realm ID: {}",
            index + 1,
            tenant.tenant_id,
            tenant.realm_id
        );
    }

    print!("Enter the number of the tenant to set as default: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= tenants.tenants.len() => {
            let tenant = &tenants.tenants[num - 1];
            match set_default_tenant(config, &tenant.tenant_id, &tenant.realm_id).await {
                Ok(_) => println!(
                    "Tenant with Tenant ID: {}, Realm ID: {} set as default.",
                    tenant.tenant_id, tenant.realm_id
                ),
                Err(e) => println!("Error setting default tenant: {}", e),
            }
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}
