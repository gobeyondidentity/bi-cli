use super::api_client::ApiClient;

use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;

pub struct Service {
    pub api_client: ApiClient,
}

impl Service {
    pub async fn new() -> Self {
        Self {
            api_client: ApiClient::new().await,
        }
    }

    pub async fn new_with_override(tenant: Tenant, realm: Realm) -> Self {
        Self {
            api_client: ApiClient::new_with_override(tenant, realm).await,
        }
    }
}
