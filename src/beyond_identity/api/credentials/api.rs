use super::types::{Credential, Credentials, CredentialsFieldName};

use crate::beyond_identity::api::common::service::CredentialsService;
use crate::beyond_identity::api::identities::types::IdentitiesFieldName;
use crate::common::error::BiError;

use function_name::named;
use http::Method;

// ====================================
// Credentials API
// ====================================

pub trait CredentialsApi {
    async fn list_credentials(
        &self,
        identity_id: &str,
        limit: Option<usize>,
    ) -> Result<Credentials, BiError>;
    async fn get_credential(
        &self,
        credential_id: &str,
        identity_id: &str,
    ) -> Result<Credential, BiError>;
    async fn revoke_credential(
        &self,
        credential_id: &str,
        identity_id: &str,
    ) -> Result<Credential, BiError>;
}

// ====================================
// Credentials API Implementation
// ====================================

impl CredentialsApi for CredentialsService {
    async fn list_credentials(
        &self,
        identity_id: &str,
        limit: Option<usize>,
    ) -> Result<Credentials, BiError> {
        let url = self
            .api_client
            .build_url()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![
                IdentitiesFieldName::Identities.name(),
                identity_id,
                CredentialsFieldName::Credentials.name(),
            ])
            .to_string()?;

        let credentials: Vec<Credential> = self
            .api_client
            .send_request_paginated(
                Method::GET,
                &url,
                None::<&()>,
                &CredentialsFieldName::Credentials.name(),
                limit,
            )
            .await?;

        Ok(Credentials {
            credentials: credentials.clone(),
            total_size: credentials.len(),
        })
    }

    async fn get_credential(
        &self,
        credential_id: &str,
        identity_id: &str,
    ) -> Result<Credential, BiError> {
        self.api_client
            .send_request(
                Method::GET,
                &self
                    .api_client
                    .build_url()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![
                        IdentitiesFieldName::Identities.name(),
                        identity_id,
                        CredentialsFieldName::Credentials.name(),
                        credential_id,
                    ])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }

    #[named]
    async fn revoke_credential(
        &self,
        credential_id: &str,
        identity_id: &str,
    ) -> Result<Credential, BiError> {
        self.api_client
            .send_request(
                Method::POST,
                &self
                    .api_client
                    .build_url()
                    .await?
                    .api()
                    .add_tenant()
                    .add_realm()
                    .add_path(vec![
                        IdentitiesFieldName::Identities.name(),
                        identity_id,
                        CredentialsFieldName::Credentials.name(),
                        credential_id,
                    ])
                    .add_custom_method(&function_name!().split('_').next().unwrap())
                    .to_string()?,
                None::<&()>,
            )
            .await
    }
}