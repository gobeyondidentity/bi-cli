use crate::beyond_identity::api::common::api_client::URLBuilder;
use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;
use crate::common::database::models::Token;
use crate::common::database::Database;
use crate::common::error::BiError;

use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::ClientWithMiddleware as Client;
use reqwest_middleware::{ClientWithMiddleware, Middleware, Next, Result as MiddlewareResult};
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
        let token = token(&self.db, &self.client, &self.tenant, &self.realm)
            .await
            .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;

        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        next.run(req, extensions).await
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

        if token.expires_at >= 0 && (token.expires_at as u64) > current_time {
            log::debug!("Using stored bearer token for all requests");
            return Ok(token.access_token);
        }
    }

    if let Some(token) = db.get_token(&tenant.id, &realm.id).await? {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

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
