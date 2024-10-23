use reqwest_middleware::ClientWithMiddleware;

use crate::{beyond_identity::tenant::TenantConfig, common::config::Config};

pub struct ApiClient {
    pub config: Config,
    pub tenant_config: TenantConfig,
    pub client: ClientWithMiddleware,
}

impl ApiClient {
    pub fn new(config: Config, tenant_config: TenantConfig, client: ClientWithMiddleware) -> Self {
        Self {
            config,
            tenant_config,
            client,
        }
    }
}
