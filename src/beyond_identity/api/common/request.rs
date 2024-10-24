use super::api_client::ApiClient;
use crate::common::error::BiError;
use http::Method;
use serde::{de::DeserializeOwned, Serialize};

pub async fn send_request<T, U>(
    api_client: &ApiClient,
    method: Method,
    url: &str,
    body: Option<&T>,
) -> Result<U, BiError>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let mut request_builder = api_client.client.request(method, url);

    if let Some(body) = body {
        request_builder = request_builder.json(body);
    }

    let response = request_builder.send().await?;
    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let response_body: U = serde_json::from_str(&response_text)?;
    Ok(response_body)
}

pub async fn send_request_paginated<T, U>(
    api_client: &ApiClient,
    method: Method,
    url: &str,
    body: Option<&T>,
    items_key: &str,
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

        // Default to maximum page size (this is likely higher than what the server accepts)
        query_params.push(format!("page_size={}", 500));

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

        let mut request_builder = api_client.client.request(method.clone(), &full_url);

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let response = request_builder.send().await?;
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(BiError::RequestError(status, response_text));
        }

        // Deserialize the response
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(items) = response_json.get(items_key) {
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
