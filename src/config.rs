use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct FilePaths {
    pub tenant_config: String,
    pub bi_scim_app_config: String,
    pub okta_scim_app_config: String,
}

impl FilePaths {
    pub fn new() -> Self {
        Self {
            tenant_config: "configs/tenant_config.json".to_string(),
            bi_scim_app_config: "configs/bi_scim_application.json".to_string(),
            okta_scim_app_config: "configs/okta_scim_application.json".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub okta_api_key: String,
    pub okta_domain: String,
    pub okta_registration_sync_attribute: String,
    pub beyond_identity_api_base_url: String,
    pub beyond_identity_auth_base_url: String,
    pub admin_display_name: String,
    pub admin_primary_email_address: String,
    pub file_paths: FilePaths,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            okta_api_key: env::var("OKTA_API_KEY").expect("OKTA_API_KEY not set"),
            okta_domain: env::var("OKTA_DOMAIN").expect("OKTA_DOMAIN not set"),
            okta_registration_sync_attribute: env::var("OKTA_REGISTRATION_SYNC_ATTRIBUTE")
                .expect("OKTA_REGISTRATION_SYNC_ATTRIBUTE not set"),
            beyond_identity_api_base_url: env::var("BEYOND_IDENTITY_API_BASE_URL")
                .expect("BEYOND_IDENTITY_API_BASE_URL not set"),
            beyond_identity_auth_base_url: env::var("BEYOND_IDENTITY_AUTH_BASE_URL")
                .expect("BEYOND_IDENTITY_AUTH_BASE_URL not set"),
            admin_display_name: env::var("ADMIN_DISPLAY_NAME").expect("ADMIN_DISPLAY_NAME not set"),
            admin_primary_email_address: env::var("ADMIN_PRIMARY_EMAIL_ADDRESS")
                .expect("ADMIN_PRIMARY_EMAIL_ADDRESS not set"),
            file_paths: FilePaths::new(),
        }
    }
}
