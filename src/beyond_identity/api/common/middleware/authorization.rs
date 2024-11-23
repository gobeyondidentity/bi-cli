use crate::beyond_identity::api::common::api_client::URLBuilder;
use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;
use crate::common::database::models::Token;
use crate::common::database::Database;
use crate::common::error::BiError;

use http::Extensions;
use http::StatusCode;
use reqwest::{Request, Response};
use reqwest_middleware::ClientWithMiddleware as Client;
use reqwest_middleware::{
    ClientWithMiddleware, Error, Middleware, Next, Result as MiddlewareResult,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthorizationMiddleware {
    db: Database,
    client: ClientWithMiddleware,
    tenant: Option<Tenant>,
    realm: Option<Realm>,
}

impl AuthorizationMiddleware {
    pub fn new(
        db: Database,
        client: ClientWithMiddleware,
        tenant: Option<Tenant>,
        realm: Option<Realm>,
    ) -> Self {
        Self {
            db,
            client,
            tenant,
            realm,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for AuthorizationMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let fetched_token = token(&self.db, &self.client, &self.tenant, &self.realm)
            .await
            .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;

        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", fetched_token).parse().unwrap(),
        );

        // Clone the request for potential retry
        let mut req_for_retry = req.try_clone().ok_or_else(|| {
            Error::Middleware(anyhow::anyhow!(
                "Request object is not clonable. Are you passing a streaming body?".to_string()
            ))
        })?;

        let mut response = next.clone().run(req, extensions).await?;

        if response.status() == StatusCode::FORBIDDEN {
            log::debug!("Received 403 Forbidden, attempting to refresh token and retry request.");

            // Invalidate the current token
            if let (Some(tenant), Some(realm)) = (&self.tenant, &self.realm) {
                self.db
                    .delete_token(&tenant.id, &realm.id)
                    .await
                    .map_err(|e| {
                        reqwest_middleware::Error::Middleware(
                            BiError::StringError(e.to_string()).into(),
                        )
                    })?;
            }

            // Fetch a new token
            let new_token = token(&self.db, &self.client, &self.tenant, &self.realm)
                .await
                .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;

            req_for_retry.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", new_token).parse().unwrap(),
            );

            response = next.run(req_for_retry, extensions).await?;

            if response.status() == StatusCode::FORBIDDEN {
                log::error!(
                    "Received 403 Forbidden after refreshing the token. This may indicate invalid credentials, insufficient permissions, or a server-side issue. Check the token, request headers, and server configuration."
                );
            }
        }

        Ok(response)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiTokenResponse {
    access_token: String,
    expires_in: u64,
}

async fn token(
    db: &Database,
    client: &Client,
    tenant: &Option<Tenant>,
    realm: &Option<Realm>,
) -> Result<String, BiError> {
    // Get tenant and realm, using defaults if not provided
    let (tenant, realm) = match (tenant, realm) {
        (Some(t), Some(r)) => (t.clone(), r.clone()),
        _ => db
            .get_default_tenant_and_realm()
            .await?
            .map(|(t, r)| (t, r))
            .ok_or_else(|| BiError::StringError("No default tenant/realm set".to_string()))?,
    };

    if let Some(token) = db.get_token(&tenant.id, &realm.id).await? {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        log::debug!(
            "Current time: {}, stored token expires at: {}",
            current_time,
            token.expires_at
        );

        if token.expires_at >= 0 && (token.expires_at as u64) > current_time {
            log::debug!("Using stored bearer token for all requests");
            return Ok(token.access_token);
        }
    }

    log::debug!("No valid token found. Fetching a new one.");

    // If no valid token, fetch a new one
    let url = URLBuilder::build(tenant.clone(), realm.clone())
        .auth()
        .add_tenant()
        .add_realm()
        .add_path(vec!["applications", &realm.application_id, "token"])
        .to_string()?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(realm.client_id, Some(realm.client_secret))
        .form(&[("grant_type", "client_credentials")])
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

    let token_response: ApiTokenResponse = serde_json::from_str(&response_text)?;

    // Calculate the expiration time
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expires_at = current_time + token_response.expires_in;

    log::debug!(
        "Token expires in: {} seconds, setting expires_at to: {}",
        token_response.expires_in,
        expires_at
    );

    let token = Token {
        access_token: token_response.access_token,
        expires_at: expires_at as i64,
        tenant_id: tenant.id,
        realm_id: realm.id,
        application_id: realm.application_id,
    };

    db.set_token(token.clone()).await?;

    Ok(token.access_token)
}
