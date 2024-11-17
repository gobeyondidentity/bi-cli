use super::types::{
    CreateCredentialBindingJobRequest, CredentialBindingJob, CredentialBindingJobEnvelope,
    CredentialBindingJobs, CredentialBindingJobsFieldName,
};

use crate::beyond_identity::api::common::service::CredentialBindingJobsService;
use crate::beyond_identity::api::identities::types::IdentitiesFieldName;
use crate::common::error::BiError;

use convert_case::{Case, Casing};
use http::Method;

// ====================================
// Credential Binding Jobs API
// ====================================

pub trait CredentialBindingJobsApi {
    async fn create_credential_binding_job(
        &self,
        identity_id: &str,
        request: &CreateCredentialBindingJobRequest,
    ) -> Result<CredentialBindingJobEnvelope, BiError>;
    async fn list_credential_binding_jobs(
        &self,
        identity_id: &str,
        limit: Option<usize>,
    ) -> Result<CredentialBindingJobs, BiError>;
    async fn get_credential_binding_job(
        &self,
        credential_binding_job_id: &str,
        identity_id: &str,
    ) -> Result<CredentialBindingJob, BiError>;
}

// ====================================
// Credential Binding Jobs API Implementation
// ====================================

impl CredentialBindingJobsApi for CredentialBindingJobsService {
    async fn create_credential_binding_job(
        &self,
        identity_id: &str,
        request: &CreateCredentialBindingJobRequest,
    ) -> Result<CredentialBindingJobEnvelope, BiError> {
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
                    .add_path(vec![
                        IdentitiesFieldName::Identities.name(),
                        identity_id,
                        &CredentialBindingJobsFieldName::CredentialBindingJobs
                            .name()
                            .to_string()
                            .to_case(Case::Kebab),
                    ])
                    .to_string()?,
                Some(request),
            )
            .await
    }

    async fn list_credential_binding_jobs(
        &self,
        identity_id: &str,
        limit: Option<usize>,
    ) -> Result<CredentialBindingJobs, BiError> {
        let url = self
            .api_client
            .builder()
            .await?
            .api()
            .add_tenant()
            .add_realm()
            .add_path(vec![
                IdentitiesFieldName::Identities.name(),
                identity_id,
                &CredentialBindingJobsFieldName::CredentialBindingJobs
                    .name()
                    .to_string()
                    .to_case(Case::Kebab),
            ])
            .to_string()?;

        let (credential_binding_jobs, total_size) = self
            .api_client
            .send_request_paginated::<_, CredentialBindingJob>(
                Method::GET,
                &url,
                None::<&()>,
                limit,
            )
            .await?;

        Ok(CredentialBindingJobs {
            credential_binding_jobs,
            total_size,
        })
    }

    async fn get_credential_binding_job(
        &self,
        credential_binding_job_id: &str,
        identity_id: &str,
    ) -> Result<CredentialBindingJob, BiError> {
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
                        IdentitiesFieldName::Identities.name(),
                        identity_id,
                        &CredentialBindingJobsFieldName::CredentialBindingJobs
                            .name()
                            .to_string()
                            .to_case(Case::Kebab),
                        credential_binding_job_id,
                    ])
                    .to_string()?,
                None::<&()>,
            )
            .await
    }
}
