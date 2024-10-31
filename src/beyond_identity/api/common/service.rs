use super::api_client::ApiClient;

use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;

pub struct Service {
    pub api_client: ApiClient,
}

pub struct ServiceBuilder {
    tenant: Option<Tenant>,
    realm: Option<Realm>,
}

impl Service {
    pub fn new() -> ServiceBuilder {
        ServiceBuilder {
            tenant: None,
            realm: None,
        }
    }
}

impl ServiceBuilder {
    pub fn tenant(mut self, tenant: Tenant) -> ServiceBuilder {
        self.tenant = Some(tenant);
        self
    }

    pub fn realm(mut self, realm: Realm) -> ServiceBuilder {
        self.realm = Some(realm);
        self
    }

    pub async fn build(self) -> Service {
        Service {
            api_client: ApiClient::new(self.tenant, self.realm).await,
        }
    }
}
