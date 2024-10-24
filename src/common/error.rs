use thiserror::Error;

#[derive(Debug, Error)]
pub enum BiError {
    #[error("Request failed with status code {0}: {1}")]
    RequestError(reqwest::StatusCode, String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    #[allow(dead_code)]
    StringError(String),
    #[error("Config file '{0}' does not exist. Please ensure the file is present at the specified path.")]
    ConfigFileNotFound(String),
    #[error("Unable to write file '{0}'")]
    UnableToWriteFile(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(url::ParseError),
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
}
