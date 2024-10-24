use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::{
    beyond_identity::tenant::TenantConfig,
    common::{config::Config, http::RespectRateLimitMiddleware},
};

use super::http::{AuthorizationMiddleware, LoggingMiddleware};

pub struct ApiClient {
    pub config: Config,
    pub tenant_config: TenantConfig,
    pub client: ClientWithMiddleware,
}

impl ApiClient {
    pub fn new(config: &Config, tenant_config: &TenantConfig) -> Self {
        let http_client = Client::new();

        let rate_limit_middleware = ClientBuilder::new(http_client.clone())
            .with(RespectRateLimitMiddleware)
            .build();

        let auth_middleware = AuthorizationMiddleware::new(
            config.clone(),
            tenant_config.clone(),
            rate_limit_middleware,
        );

        let client = ClientBuilder::new(http_client)
            .with(auth_middleware)
            .with(LoggingMiddleware)
            .with(RespectRateLimitMiddleware)
            .build();

        Self {
            config: config.clone(),
            tenant_config: tenant_config.clone(),
            client,
        }
    }
}
