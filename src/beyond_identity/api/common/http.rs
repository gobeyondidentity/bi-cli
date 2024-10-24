use super::token::token;
use crate::beyond_identity::tenant::TenantConfig;
use crate::common::config::Config;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientWithMiddleware, Middleware, Next, Result};

pub struct AuthorizationMiddleware {
    config: Config,
    tenant_config: TenantConfig,
    client: ClientWithMiddleware,
}

impl AuthorizationMiddleware {
    pub fn new(config: Config, tenant_config: TenantConfig, client: ClientWithMiddleware) -> Self {
        Self {
            config,
            tenant_config,
            client,
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
    ) -> Result<Response> {
        let token = token(&self.client, &self.config, &self.tenant_config)
            .await
            .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;

        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        next.run(req, extensions).await
    }
}

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        log::debug!(
            "Sending request: method = {:?}, url = {}, body = {:?}",
            req.method(),
            req.url(),
            req.body().map(|b| format!("{:?}", b))
        );

        let result = next.run(req, extensions).await;

        match &result {
            Ok(response) => {
                log::debug!("Received response: status = {}", response.status());
            }
            Err(err) => {
                log::error!("Request error: {:?}", err);
            }
        }

        result
    }
}
