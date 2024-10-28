use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;
use crate::common::database::Database;
use crate::common::error::BiError;
use crate::setup::tenants::application::get_management_api_application;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // The issuer field in the JWT
}

pub async fn provision_tenant(
    client: &Client,
    db: &Database,
    token: &str,
) -> Result<(Tenant, Realm), BiError> {
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

    let tenant = Tenant {
        id: tenant_id.clone(),
    };

    let realm = Realm {
        id: realm_id.clone(),
        tenant_id: tenant_id.clone(),
        application_id,
        client_id,
        client_secret,
        open_id_configuration_url: format!("{}/.well-known/openid-configuration", issuer_url),
        api_base_url,
        auth_base_url,
    };

    let tenants_with_realms = db.get_all_tenants_with_realms().await?;

    // Check for existing tenant-realm combination to avoid duplicates
    if tenants_with_realms
        .iter()
        .any(|(t, realms)| t.id == tenant_id && realms.iter().any(|r| r.id == realm_id))
    {
        return Err(BiError::StringError(
            "Tenant/realm already provisioned".to_string(),
        ));
    }

    db.set_tenant_and_realm(tenant.clone(), realm.clone())
        .await?;

    // Check if there is already a default tenant and realm
    if db.get_default_tenant_and_realm().await?.is_none() {
        // Set this tenant and realm as the default
        db.set_default_tenant_and_realm(&tenant_id, &realm_id)
            .await?;
    }

    Ok((tenant.clone(), realm.clone()))
}

pub async fn list_tenants_ui(db: &Database) -> Result<(), BiError> {
    // Fetch all tenants with their corresponding realms
    let tenants_with_realms = db.get_all_tenants_with_realms().await?;
    if tenants_with_realms.is_empty() {
        println!("No tenants found.");
    } else {
        println!("List of Tenants:");

        // Fetch the current default tenant and realm
        let default_tenant_realm = db.get_default_tenant_and_realm().await?;

        for (index, (tenant, realms)) in tenants_with_realms.iter().enumerate() {
            for realm in realms {
                // Determine if this tenant/realm is the default
                let default_marker =
                    if let Some((default_tenant, default_realm)) = &default_tenant_realm {
                        if default_tenant.id == tenant.id && default_realm.id == realm.id {
                            "(Default)"
                        } else {
                            ""
                        }
                    } else {
                        ""
                    };

                println!(
                    "{}: Tenant ID: {}, Realm ID: {} {}",
                    index + 1,
                    tenant.id,
                    realm.id,
                    default_marker
                );
            }
        }
    }
    Ok(())
}

pub async fn delete_tenant_ui(db: &Database) -> Result<(), BiError> {
    // Retrieve all tenants with realms from the database
    let tenants_with_realms = db.get_all_tenants_with_realms().await?;
    if tenants_with_realms.is_empty() {
        println!("No tenants to delete.");
        return Ok(());
    }

    // Display tenants and their corresponding realms for selection
    println!("Select a tenant/realm pair to delete:");
    for (index, (tenant, realms)) in tenants_with_realms.iter().enumerate() {
        for realm in realms {
            println!(
                "{}: Tenant ID: {}, Realm ID: {}",
                index + 1,
                tenant.id,
                realm.id
            );
        }
    }

    // Get user input for the selection
    print!("Enter the number of the tenant/realm to delete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= tenants_with_realms.len() => {
            // Find selected tenant and realm
            let (tenant, realms) = &tenants_with_realms[num - 1];
            let realm = &realms[0]; // Selecting the first realm for this example

            // Call the delete function
            match db.delete_tenant_realm_pair(&tenant.id, &realm.id).await {
                Ok(_) => println!(
                    "Tenant with Tenant ID: {}, Realm ID: {} deleted.",
                    tenant.id, realm.id
                ),
                Err(e) => println!("Error deleting tenant/realm: {}", e),
            }
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}

pub async fn set_default_tenant_ui(db: &Database) -> Result<(), BiError> {
    // Fetch all tenants with realms from the database
    let tenants_with_realms = db.get_all_tenants_with_realms().await?;

    if tenants_with_realms.is_empty() {
        println!("No tenants available to set as default.");
        return Ok(());
    }

    println!("Select a tenant to set as default:");
    for (index, (tenant, realms)) in tenants_with_realms.iter().enumerate() {
        for realm in realms {
            println!(
                "{}: Tenant ID: {}, Realm ID: {}",
                index + 1,
                tenant.id,
                realm.id
            );
        }
    }

    print!("Enter the number of the tenant to set as default: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= tenants_with_realms.len() => {
            let (tenant, realm) = &tenants_with_realms[num - 1];
            match db
                .set_default_tenant_and_realm(&tenant.id, &realm[0].id)
                .await
            {
                Ok(_) => println!(
                    "Tenant with Tenant ID: {}, Realm ID: {} set as default.",
                    tenant.id, realm[0].id
                ),
                Err(e) => println!("Error setting default tenant: {}", e),
            }
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}
