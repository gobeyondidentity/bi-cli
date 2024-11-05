use crate::common::error::BiError;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    id: String,
    realm_id: String,
    tenant_id: String,
    display_name: String,
    is_managed: bool,
    classification: String,
    pub protocol_config: ProtocolConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApplicationsResponse {
    applications: Vec<Application>,
    total_size: u32,
    next_page_token: Option<String>,
}

pub async fn get_management_api_application(
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
