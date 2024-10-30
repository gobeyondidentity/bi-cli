use crate::beyond_identity::api;
use crate::beyond_identity::api::realms::api::RealmsApi;
use crate::beyond_identity::api::{common::service::Service, tenants::api::TenantsApi};
use crate::common::database;
use crate::common::database::Database;
use crate::common::error::BiError;
use crate::setup::tenants::application::get_management_api_application;

use futures::future::join_all;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tabled::settings::object::Rows;
use tabled::settings::themes::Colorization;
use tabled::settings::{Color, Merge};
use tabled::{
    settings::style::{BorderSpanCorrection, Style},
    Table, Tabled,
};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // The issuer field in the JWT
}

pub async fn provision_tenant(
    client: &Client,
    db: &Database,
    token: &str,
) -> Result<(database::models::Tenant, database::models::Realm), BiError> {
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

    let tenant = database::models::Tenant {
        id: tenant_id.clone(),
    };

    let realm = database::models::Realm {
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

    _ = display(db).await?;

    Ok((tenant.clone(), realm.clone()))
}

pub async fn list_tenants_ui(db: &Database) -> Result<(), BiError> {
    _ = display(db).await?;
    Ok(())
}

pub async fn delete_tenant_ui(db: &Database) -> Result<(), BiError> {
    let tenants_with_realms = match display(db).await? {
        Some(x) => x,
        None => return Ok(()),
    };

    let tenants_with_realms = flatten(&tenants_with_realms)?;

    // Get user input for the selection
    print!("Enter the number of the tenant/realm to remove: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= tenants_with_realms.len() => {
            // Find selected tenant and realm
            let (tenant, realm) = &tenants_with_realms[num - 1];
            match db.delete_tenant_realm_pair(&tenant.id, &realm.id).await {
                Ok(_) => _ = display(db).await?,
                Err(e) => println!("Error deleting tenant/realm: {}", e),
            }
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}

pub async fn set_default_tenant_ui(db: &Database) -> Result<(), BiError> {
    let tenants_with_realms = match display(db).await? {
        Some(x) => x,
        None => return Ok(()),
    };

    let tenants_with_realms = flatten(&tenants_with_realms)?;

    print!("Enter the number of the tenant to set as default: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= tenants_with_realms.len() => {
            let (tenant, realm) = &tenants_with_realms[num - 1];
            match db.set_default_tenant_and_realm(&tenant.id, &realm.id).await {
                Ok(_) => _ = display(db).await?,
                Err(e) => println!("Error setting default tenant: {}", e),
            }
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}

#[derive(Tabled)]
struct RealmDisplay {
    #[tabled(rename = "Index")]
    index: String,
    #[tabled(rename = "Tenant Name")]
    tenant_name: String,
    #[tabled(rename = "Tenant ID")]
    tenant_id: String,
    #[tabled(rename = "Realm Name")]
    realm_name: String,
    #[tabled(rename = "Realm ID")]
    realm_id: String,
}

async fn display(
    db: &Database,
) -> Result<Option<Vec<(database::models::Tenant, Vec<database::models::Realm>)>>, BiError> {
    // Fetch all tenants with their corresponding realms
    let tenants_with_realms = db.get_all_tenants_with_realms().await?;
    if tenants_with_realms.is_empty() {
        println!("No tenants found.");
        return Ok(None);
    } else {
        // Fetch the current default tenant and realm
        let (default_tenant, default_realm) = match db.get_default_tenant_and_realm().await? {
            Some((t, r)) => (t, r),
            None => {
                return Err(BiError::StringError(
                    "No default tenant/realm set".to_string(),
                ))
            }
        };

        let fetched_data = join_all(tenants_with_realms.iter().flat_map(|(tenant, realms)| {
            realms.iter().map(move |realm| {
                let tenant = tenant.clone();
                let realm = realm.clone();
                async move {
                    let result = get_fully_resolved_tenant_and_realm(&tenant, &realm).await;
                    (tenant, realm, result)
                }
            })
        }))
        .await;

        let mut display = vec![];
        let mut index = 1;

        for (tenant, realm, result) in fetched_data {
            let (api_tenant, api_realm) = result?;
            let is_default = default_tenant.id == tenant.id && default_realm.id == realm.id;
            let environment = match realm.api_base_url.split('.').last().unwrap_or_default() {
                "run" => "[r]".to_string(),
                "xyz" => "[s]".to_string(),
                "dev" => "[d]".to_string(),
                "com" => "".to_string(),
                _ => "[?]".to_string(),
            };
            display.push(RealmDisplay {
                index: if is_default {
                    format!("{}* {}", index, environment)
                } else {
                    format!("{}  {}", index, environment)
                },
                tenant_name: api_tenant.display_name.clone(),
                tenant_id: tenant.id.clone(),
                realm_name: api_realm.display_name.clone(),
                realm_id: realm.id.clone(),
            });

            index += 1;
        }

        let mut table = Table::new(display);
        table.with(Style::extended());
        table.with(Merge::vertical());
        table.with(BorderSpanCorrection);

        // Define colors
        let color_rolling = Color::BG_YELLOW | Color::FG_BLACK;
        let color_staging = Color::BG_GREEN | Color::FG_BLACK;
        let color_development = Color::BG_RED | Color::FG_BLACK;
        let color_default = Color::default();

        let mut row_index = 1; // Start from 1 to skip header row

        for (_, realms) in tenants_with_realms.iter() {
            for realm in realms {
                let row_color = match realm.api_base_url.split('.').last().unwrap_or_default() {
                    "run" => color_rolling.clone(),
                    "xyz" => color_staging.clone(),
                    "dev" => color_development.clone(),
                    _ => color_default.clone(),
                };
                table.with(Colorization::exact([row_color], Rows::single(row_index)));
                row_index += 1;
            }
        }

        println!("{}", table);
        Ok(Some(tenants_with_realms))
    }
}

async fn get_fully_resolved_tenant_and_realm(
    tenant: &database::models::Tenant,
    realm: &database::models::Realm,
) -> Result<(api::tenants::types::Tenant, api::realms::types::Realm), BiError> {
    let api_tenant = Service::new_with_override(tenant.clone(), realm.clone())
        .await
        .get_tenant()
        .await?;

    let api_realm = Service::new_with_override(tenant.clone(), realm.clone())
        .await
        .get_realm(&realm.id)
        .await?;

    return Ok((api_tenant, api_realm));
}

fn flatten(
    input: &Vec<(database::models::Tenant, Vec<database::models::Realm>)>,
) -> Result<Vec<(database::models::Tenant, database::models::Realm)>, BiError> {
    let mut flattened = Vec::new();
    for (tenant, realms) in input {
        for realm in realms {
            flattened.push((tenant.clone(), realm.clone()));
        }
    }
    Ok(flattened)
}
