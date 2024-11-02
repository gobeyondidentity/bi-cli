use std::usize;

use super::middleware::authorization::AuthorizationMiddleware;
use super::middleware::logging::LoggingMiddleware;
use super::middleware::rate_limit::RespectRateLimitMiddleware;

use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;
use crate::common::{database::Database, error::BiError};

use http::Method;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::de::{self, Deserializer, MapAccess};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;
use url::Url;

pub struct ApiClient {
    pub client: ClientWithMiddleware,
    pub db: Database,
    tenant: Option<Tenant>,
    realm: Option<Realm>,
}

impl ApiClient {
    pub async fn new(tenant: Option<Tenant>, realm: Option<Realm>) -> Self {
        let db = Database::initialize().await.unwrap();

        let http_client = Client::new();

        let rate_limit_middleware = ClientBuilder::new(http_client.clone())
            .with(RespectRateLimitMiddleware)
            .build();

        let auth_middleware = AuthorizationMiddleware::new(
            db.clone(),
            rate_limit_middleware,
            tenant.clone(),
            realm.clone(),
        );

        let client = ClientBuilder::new(http_client)
            .with(auth_middleware)
            .with(LoggingMiddleware)
            .with(RespectRateLimitMiddleware)
            .build();

        Self {
            client,
            db,
            tenant,
            realm,
        }
    }

    // Initializes the URLBuilder
    pub async fn builder(&self) -> Result<URLBuilder, BiError> {
        // Get tenant and realm, using defaults if not provided
        let (tenant, realm) = match (self.tenant.clone(), self.realm.clone()) {
            (Some(t), Some(r)) => (t.clone(), r.clone()),
            _ => self
                .db
                .get_default_tenant_and_realm()
                .await?
                .map(|(t, r)| (t, r))
                .ok_or_else(|| BiError::StringError("No default tenant/realm set".to_string()))?,
        };

        Ok(URLBuilder::build(tenant, realm))
    }

    pub async fn send_request<T, U>(
        &self,
        method: Method,
        url: &str,
        body: Option<&T>,
    ) -> Result<U, BiError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let mut request_builder = self.client.request(method, url);

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
        &self,
        method: Method,
        url: &str,
        body: Option<&T>,
        limit: Option<usize>,
    ) -> Result<(Vec<U>, usize), BiError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let mut final_results = Vec::new();
        let mut next_page_token: Option<String> = None;
        let mut remaining_limit = limit.unwrap_or(usize::MAX);
        // This will be set on first page response
        let mut total_size: usize = 0;

        loop {
            if remaining_limit == 0 {
                return Ok((final_results, total_size));
            }

            // Construct the full URL, including pagination if applicable
            let mut full_url = url.to_string();
            let mut query_params = vec![];

            let page_size = remaining_limit.min(500);
            query_params.push(format!("page_size={}", page_size));

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

            let response: PaginatedResponse<U> =
                self.send_request(method.clone(), &full_url, body).await?;

            total_size = response.total_size;

            if response.items.len() > 0 {
                remaining_limit = remaining_limit.saturating_sub(response.items.len());
                final_results.extend(response.items);
            }

            next_page_token = response.next_page_token;

            // Break if there's no next page
            if next_page_token.is_none() {
                break;
            }
        }

        Ok((final_results, total_size))
    }
}

#[derive(Debug)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    total_size: usize,
    next_page_token: Option<String>,
}

impl<'de, T> Deserialize<'de> for PaginatedResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<PaginatedResponse<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = PaginatedResponse<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a paginated response")
            }

            fn visit_map<M>(self, mut map: M) -> Result<PaginatedResponse<T>, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut items = None;
                let mut total_size = None;
                let mut next_page_token = None;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "total_size" => total_size = Some(map.next_value()?),
                        "next_page_token" => next_page_token = map.next_value()?,
                        _ if items.is_none() => {
                            let value: Result<Vec<T>, _> = map.next_value();
                            if let Ok(v) = value {
                                items = Some(v);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(PaginatedResponse {
                    items: items
                        .ok_or_else(|| de::Error::custom("No items array found in response"))?,
                    total_size: total_size.ok_or_else(|| de::Error::missing_field("total_size"))?,
                    next_page_token,
                })
            }
        }

        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}

pub struct URLBuilder {
    url: Option<Url>,
    api_base_url: String,
    auth_base_url: String,
    tenant_id: String,
    realm_id: String,
}

impl URLBuilder {
    // Initializes the URLBuilder
    pub fn build(tenant: Tenant, realm: Realm) -> Self {
        Self {
            url: None,
            api_base_url: realm.api_base_url,
            auth_base_url: realm.auth_base_url,
            tenant_id: tenant.id,
            realm_id: realm.id,
        }
    }

    /// Specifies that this is an API URL.
    pub fn api(mut self) -> Self {
        self.url = Some(Url::parse(&self.api_base_url).expect("Invalid API base URL"));
        self
    }

    /// Specifies that this is an Auth URL.
    pub fn auth(mut self) -> Self {
        self.url = Some(Url::parse(&self.auth_base_url).expect("Invalid Auth base URL"));
        self
    }

    /// Helper method to get a mutable reference to the URL.
    fn url_mut(&mut self) -> &mut Url {
        self.url
            .as_mut()
            .expect("URL not initialized. Call api() or auth() first.")
    }

    /// Adds the tenant ID to the URL path.
    pub fn add_tenant(mut self) -> Self {
        let tenant_id = self.tenant_id.clone();
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(&["v1", "tenants", &tenant_id]);
        self
    }

    /// Adds the realm ID to the URL path.
    pub fn add_realm(mut self) -> Self {
        let realm_id = self.realm_id.clone();
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(&["realms", &realm_id]);
        self
    }

    /// Adds the specified realm ID to the URL path.
    pub fn add_realm_with_override(mut self, id: String) -> Self {
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(&["realms", &id]);
        self
    }

    /// Adds additional path segments to the URL.
    pub fn add_path(mut self, segments: Vec<&str>) -> Self {
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(segments.iter().copied());
        self
    }

    /// Appends a colon suffix to the last path segment.
    pub fn add_custom_method(mut self, suffix: &str) -> Self {
        // Get the current path segments as a vector of strings.
        let mut segments: Vec<String> = self
            .url_mut()
            .path_segments()
            .map(|segments| segments.map(|s| s.to_string()).collect())
            .unwrap_or_default();

        if let Some(last_segment) = segments.last_mut() {
            // Append the colon suffix to the last segment.
            *last_segment = format!("{}:{}", last_segment, suffix);
        } else {
            // If there are no segments, add the suffix as a new segment.
            segments.push(format!(":{}", suffix));
        }

        // Clear existing path segments and set the modified ones.
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .clear()
            .extend(segments.iter().map(|s| &**s));

        self
    }

    /// Adds a query parameter to the URL if the value is `Some`.
    pub fn add_query_param(mut self, key: &str, value: Option<&str>) -> Self {
        if let Some(value) = value {
            self.url_mut().query_pairs_mut().append_pair(key, value);
        }
        self
    }

    /// Converts the URLBuilder into a `String` representing the final URL.
    pub fn to_string(self) -> Result<String, BiError> {
        self.url
            .ok_or_else(|| {
                BiError::StringError("URL not initialized. Call api() or auth() first.".into())
            })
            .map(|url| url.to_string())
    }
}
