use super::command::ListFieldName;
use super::types::{CreateRealmRequest, PatchRealmRequest, Realm, Realms, RealmsFieldName};

use crate::beyond_identity::api::common::filter::Filter;
use crate::beyond_identity::api::common::service::RealmsService;
use crate::common::error::BiError;

use http::Method;

// ====================================
// Realms API
// ====================================

pub trait RealmsApi {
    async fn create_realm(&self, request: &CreateRealmRequest) -> Result<Realm, BiError>;
    async fn list_realms(
        &self,
        filter: Option<Filter>,
        limit: Option<usize>,
    ) -> Result<Realms, BiError>;
    async fn get_realm(&self, realm_id: &str) -> Result<Realm, BiError>;
    async fn patch_realm(&self, request: &PatchRealmRequest) -> Result<Realm, BiError>;
    async fn delete_realm(&self, realm_id: &str) -> Result<serde_json::Value, BiError>;
}

// ====================================
// Realms API Implementation
// ====================================

impl RealmsApi for RealmsService {
    async fn create_realm(&self, request: &CreateRealmRequest) -> Result<Realm, BiError> {
        self.api_client
            .send_request(
                Method::POST,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_path(vec![RealmsFieldName::Realms.name()])
                    .to_string()?,
                Some(request),
            )
            .await
    }

    async fn list_realms(
        &self,
        filter: Option<Filter>,
        limit: Option<usize>,
    ) -> Result<Realms, BiError> {
        let url = self
            .api_client
            .builder()
            .await?
            .api()
            .add_tenant()
            .add_path(vec![RealmsFieldName::Realms.name()])
            .add_query_param(
                &ListFieldName::Filter.name(),
                filter.as_ref().map(|f| f.0.as_ref()),
            )
            .to_string()?;

        let (realms, total_size) = self
            .api_client
            .send_request_paginated::<_, Realm>(Method::GET, &url, None::<&()>, limit, None)
            .await?;

        Ok(Realms { realms, total_size })
    }

    async fn get_realm(&self, realm_id: &str) -> Result<Realm, BiError> {
        self.api_client
            .send_request(
                Method::GET,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm_with_override(realm_id.to_string())
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    async fn patch_realm(&self, request: &PatchRealmRequest) -> Result<Realm, BiError> {
        self.api_client
            .send_request(
                Method::PATCH,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .to_string()?,
                Some(request),
            )
            .await
    }

    async fn delete_realm(&self, realm_id: &str) -> Result<serde_json::Value, BiError> {
        self.api_client
            .send_request(
                Method::DELETE,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm_with_override(realm_id.to_string())
                    .to_string()?,
                None::<&()>,
            )
            .await
    }
}
