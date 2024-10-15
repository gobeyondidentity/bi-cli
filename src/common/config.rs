use crate::common::error::BiError;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FilePaths {
    pub tenant_config: String,
    pub bi_scim_app_config: String,
    pub okta_scim_app_config: String,
    pub external_sso_config: String,
    pub okta_identity_provider: String,
    pub okta_routing_rule: String,
    pub okta_custom_attribute: String,
    pub okta_applications: String,
    pub onelogin_applications: String,
    pub token_path: String,
    okta_config_path: String,
    onelogin_config_path: String,
}

impl FilePaths {
    pub fn new() -> Self {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let base_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        let configs_dir = base_dir.join("configs");

        if !configs_dir.exists() {
            fs::create_dir_all(&configs_dir).expect("Failed to create configs directory");
        }

        fn path_to_string(path: PathBuf) -> String {
            path.to_str()
                .expect("Failed to convert path to string")
                .to_string()
        }

        Self {
            tenant_config: path_to_string(configs_dir.join("tenant_config.json")),
            bi_scim_app_config: path_to_string(configs_dir.join("bi_scim_application.json")),
            okta_scim_app_config: path_to_string(configs_dir.join("okta_scim_application.json")),
            external_sso_config: path_to_string(configs_dir.join("external_sso.json")),
            okta_identity_provider: path_to_string(configs_dir.join("okta_identity_provider.json")),
            okta_routing_rule: path_to_string(configs_dir.join("okta_routing_rule.json")),
            okta_custom_attribute: path_to_string(configs_dir.join("okta_custom_attribute.json")),
            okta_applications: path_to_string(configs_dir.join("okta_applications.json")),
            onelogin_applications: path_to_string(configs_dir.join("onelogin_applications.json")),
            token_path: path_to_string(configs_dir.join("token.json")),
            okta_config_path: path_to_string(configs_dir.join("okta_config.json")),
            onelogin_config_path: path_to_string(configs_dir.join("onelogin_config.json")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub file_paths: FilePaths,
}

impl Config {
    pub fn new() -> Self {
        Config {
            file_paths: FilePaths::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OktaConfig {
    pub domain: String,
    pub api_key: String,
}

impl OktaConfig {
    pub fn new() -> Result<Self, BiError> {
        Self::load_from_file()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OneloginConfig {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
}

impl OneloginConfig {
    pub fn new() -> Result<Self, BiError> {
        Self::load_from_file()
    }
}

impl OktaConfig {
    pub fn save_to_file(&self) -> Result<(), BiError> {
        let serialized = serde_json::to_string_pretty(self).map_err(BiError::SerdeError)?;
        let config_path = FilePaths::new().okta_config_path;

        let config_dir = std::path::Path::new(&config_path)
            .parent()
            .ok_or_else(|| BiError::UnableToWriteFile(config_path.clone()))?;
        fs::create_dir_all(config_dir)
            .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

        fs::write(&config_path, serialized)
            .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

        Ok(())
    }

    pub fn load_from_file() -> Result<Self, BiError> {
        let file_paths = FilePaths::new();
        let config_path = &file_paths.okta_config_path;
        let data = fs::read_to_string(&config_path)
            .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
        let okta_config: OktaConfig = serde_json::from_str(&data).map_err(BiError::SerdeError)?;
        Ok(okta_config)
    }
}

impl OneloginConfig {
    pub fn save_to_file(&self) -> Result<(), BiError> {
        let serialized = serde_json::to_string_pretty(self).map_err(BiError::SerdeError)?;
        let config_path = FilePaths::new().onelogin_config_path;

        let config_dir = std::path::Path::new(&config_path)
            .parent()
            .ok_or_else(|| BiError::UnableToWriteFile(config_path.clone()))?;
        fs::create_dir_all(config_dir)
            .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

        fs::write(&config_path, serialized)
            .map_err(|_| BiError::UnableToWriteFile(config_path.clone()))?;

        Ok(())
    }

    pub fn load_from_file() -> Result<Self, BiError> {
        let file_paths = FilePaths::new();
        let config_path = &file_paths.onelogin_config_path;
        let data = fs::read_to_string(&config_path)
            .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
        let onelogin_config: OneloginConfig =
            serde_json::from_str(&data).map_err(BiError::SerdeError)?;
        Ok(onelogin_config)
    }
}
