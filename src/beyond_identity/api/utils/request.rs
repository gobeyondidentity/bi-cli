use http::Method;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    beyond_identity::{api_token::get_beyond_identity_api_token, tenant::TenantConfig},
    common::{config::Config, error::BiError},
};

pub async fn send_request<T, U>(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    method: Method,
    url: &str,
    body: Option<&T>,
) -> Result<U, BiError>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let mut request_builder = client.request(method, url).header(
        "Authorization",
        format!(
            "Bearer {}",
            get_beyond_identity_api_token(client, config, tenant_config).await?
        ),
    );

    if let Some(body) = body {
        request_builder = request_builder.json(body);
    }

    let response = request_builder.send().await?;
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(BiError::RequestError(status, error_text));
    }

    let response_body: U = response.json().await?;
    Ok(response_body)
}

pub async fn send_request_paginated<T, U>(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    method: Method,
    url: &str,
    page_size: Option<u32>,
    body: Option<&T>,
) -> Result<Vec<U>, BiError>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let mut final_results = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        // Construct the full URL, including pagination if applicable
        let mut full_url = url.to_string();
        let mut query_params = vec![];

        // Add page size if provided
        if let Some(size) = page_size {
            query_params.push(format!("page_size={}", size));
        }

        // Add next_page_token if available
        if let Some(ref token) = next_page_token {
            query_params.push(format!("page_token={}", token));
        }

        // Append query parameters to the URL if any
        if !query_params.is_empty() {
            let query_string = query_params.join("&");
            if full_url.contains('?') {
                full_url.push_str(&format!("&{}", query_string));
            } else {
                full_url.push_str(&format!("?{}", query_string));
            }
        }

        let mut request_builder = client.request(method.clone(), &full_url).header(
            "Authorization",
            format!(
                "Bearer {}",
                get_beyond_identity_api_token(client, config, tenant_config).await?
            ),
        );

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let response = request_builder.send().await?;
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(BiError::RequestError(status, error_text));
        }

        // Deserialize the response
        let response_json: serde_json::Value = response.json().await?;
        if let Some(items) = response_json.get("items") {
            let mut page_results: Vec<U> = serde_json::from_value(items.clone())?;
            final_results.append(&mut page_results);
        }

        // Check for a next page token
        next_page_token = response_json
            .get("next_page_token")
            .and_then(|token| token.as_str().map(String::from));

        // Break if there's no next page
        if next_page_token.is_none() {
            break;
        }
    }

    Ok(final_results)
}
