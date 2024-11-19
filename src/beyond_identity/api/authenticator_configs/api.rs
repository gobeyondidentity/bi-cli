use super::types::{
    AuthenticatorConfig, AuthenticatorConfigs, AuthenticatorConfigsFieldName,
    CreateAuthenticatorConfigRequest, OptionalAuthenticatorConfigRequest,
    PatchAuthenticatorConfigRequest,
};

use crate::beyond_identity::api::common::service::AuthenticatorConfigsService;
use crate::common::error::BiError;

use convert_case::{Case, Casing};
use http::Method;

// ====================================
// Authenticator Configs API
// ====================================

pub trait AuthenticatorConfigsApi {
    async fn create_authenticator_config(
        &self,
        request: &CreateAuthenticatorConfigRequest,
    ) -> Result<AuthenticatorConfig, BiError>;
    async fn list_authenticator_configs(
        &self,
        limit: Option<usize>,
    ) -> Result<AuthenticatorConfigs, BiError>;
    async fn get_authenticator_config(
        &self,
        authenticator_config_id: &str,
    ) -> Result<AuthenticatorConfig, BiError>;
    async fn patch_authenticator_config(
        &self,
        request: &PatchAuthenticatorConfigRequest,
    ) -> Result<AuthenticatorConfig, BiError>;
    async fn delete_authenticator_config(
        &self,
        authenticator_config_id: &str,
    ) -> Result<serde_json::Value, BiError>;
}

// ====================================
// Authenticator Configs API Implementation
// ====================================

impl AuthenticatorConfigsApi for AuthenticatorConfigsService {
    async fn create_authenticator_config(
        &self,
        request: &CreateAuthenticatorConfigRequest,
    ) -> Result<AuthenticatorConfig, BiError> {
        let config = OptionalAuthenticatorConfigRequest::from(request);
        self.api_client
            .send_request(
                Method::POST,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![&AuthenticatorConfigsFieldName::AuthenticatorConfigs
                        .name()
                        .to_string()
                        .to_case(Case::Kebab)])
                    .to_string()?,
                Some(&config),
            )
            .await
    }

    async fn list_authenticator_configs(
        &self,
        limit: Option<usize>,
    ) -> Result<AuthenticatorConfigs, BiError> {
        let url = self
            .api_client
            .builder()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![&AuthenticatorConfigsFieldName::AuthenticatorConfigs
                .name()
                .to_string()
                .to_case(Case::Kebab)])
            .to_string()?;

        let (authenticator_configs, total_size) = self
            .api_client
            .send_request_paginated::<_, AuthenticatorConfig>(
                Method::GET,
                &url,
                None::<&()>,
                limit,
                Some(100),
            )
            .await?;

        Ok(AuthenticatorConfigs {
            authenticator_configs,
            total_size,
        })
    }

    async fn get_authenticator_config(
        &self,
        authenticator_config_id: &str,
    ) -> Result<AuthenticatorConfig, BiError> {
        self.api_client
            .send_request(
                Method::GET,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![
                        &AuthenticatorConfigsFieldName::AuthenticatorConfigs
                            .name()
                            .to_string()
                            .to_case(Case::Kebab),
                        authenticator_config_id,
                    ])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    async fn patch_authenticator_config(
        &self,
        request: &PatchAuthenticatorConfigRequest,
    ) -> Result<AuthenticatorConfig, BiError> {
        let config = OptionalAuthenticatorConfigRequest::from(request);
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
                    .add_path(vec![
                        &AuthenticatorConfigsFieldName::AuthenticatorConfigs
                            .name()
                            .to_string()
                            .to_case(Case::Kebab),
                        &config
                            .clone()
                            .authenticator_config
                            .id
                            .ok_or(BiError::StringError(
                                "ID required when patching an authenticator config".to_string(),
                            ))?,
                    ])
                    .to_string()?,
                Some(&config),
            )
            .await
    }

    async fn delete_authenticator_config(
        &self,
        authenticator_config_id: &str,
    ) -> Result<serde_json::Value, BiError> {
        self.api_client
            .send_request(
                Method::DELETE,
                &self
                    .api_client
                    .builder()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![
                        &AuthenticatorConfigsFieldName::AuthenticatorConfigs
                            .name()
                            .to_string()
                            .to_case(Case::Kebab),
                        authenticator_config_id,
                    ])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }
}
