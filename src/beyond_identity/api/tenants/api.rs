use http::Method;

use super::types::{PatchTenantRequest, Tenant};

use crate::beyond_identity::api::common::service::Service;
use crate::beyond_identity::api::common::url::URLBuilder;
use crate::common::error::BiError;

// ====================================
// Tenants API
// ====================================

pub trait TenantsApi {
    async fn get_tenant(&self) -> Result<Tenant, BiError>;
    async fn patch_tenant(&self, patch_request: &PatchTenantRequest) -> Result<Tenant, BiError>;
}

// ====================================
// Tenants API Implementation
// ====================================

impl TenantsApi for Service {
    async fn get_tenant(&self) -> Result<Tenant, BiError> {
        self.api_client
            .send_request(
                Method::GET,
                &URLBuilder::build(&self.api_client.tenant_config)
                    .api()
                    .add_tenant()
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    async fn patch_tenant(&self, patch_request: &PatchTenantRequest) -> Result<Tenant, BiError> {
        self.api_client
            .send_request(
                Method::PATCH,
                &URLBuilder::build(&self.api_client.tenant_config)
                    .api()
                    .add_tenant()
                    .to_string()?,
                Some(patch_request),
            )
            .await
    }
}
