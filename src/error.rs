use thiserror::Error;

#[derive(Debug, Error)]
pub enum BiError {
    #[error("Request failed with status code {0}: {1}")]
    RequestError(reqwest::StatusCode, String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    StringError(String),
}
