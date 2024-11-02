use super::types::{PatchTenantRequest, Tenant};

use crate::beyond_identity::api::common::service::TenantsService;
use crate::common::error::BiError;

use http::Method;

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

impl TenantsApi for TenantsService {
    async fn get_tenant(&self) -> Result<Tenant, BiError> {
        self.api_client
            .send_request(
                Method::GET,
                &self
                    .api_client
                    .builder()
                    .await?
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
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .to_string()?,
                Some(patch_request),
            )
            .await
    }
}
