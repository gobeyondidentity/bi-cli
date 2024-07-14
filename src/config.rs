use dotenv::dotenv;
use regex::Regex;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct FilePaths {
    pub tenant_config: String,
    pub bi_scim_app_config: String,
    pub okta_scim_app_config: String,
    pub external_sso_config: String,
    pub okta_identity_provider: String,
    pub okta_routing_rule: String,
    pub okta_custom_attribute: String,
    pub okta_applications: String,
}

impl FilePaths {
    pub fn new() -> Self {
        Self {
            tenant_config: "configs/tenant_config.json".to_string(),
            bi_scim_app_config: "configs/bi_scim_application.json".to_string(),
            okta_scim_app_config: "configs/okta_scim_application.json".to_string(),
            external_sso_config: "configs/external_sso.json".to_string(),
            okta_identity_provider: "configs/okta_identity_provider.json".to_string(),
            okta_routing_rule: "configs/okta_routing_rule.json".to_string(),
            okta_custom_attribute: "configs/okta_custom_attribute.json".to_string(),
            okta_applications: "configs/okta_applications.json".to_string(),
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
        validate_env().expect("Environment validation failed");

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

fn validate_env() -> Result<(), String> {
    let env_vars: HashMap<String, String> = env::vars().collect();
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    let valid_api_urls = vec![
        "https://api-us.beyondidentity.run",
        "https://api-us.beyondidentity.xyz",
        "https://api-us.beyondidentity.com",
    ];
    let valid_auth_urls = vec![
        "https://auth-us.beyondidentity.run",
        "https://auth-us.beyondidentity.xyz",
        "https://auth-us.beyondidentity.com",
    ];

    // Validate OKTA_DOMAIN
    if let Some(okta_domain) = env_vars.get("OKTA_DOMAIN") {
        if !okta_domain.starts_with("https://") || okta_domain.ends_with('/') {
            return Err(format!(
                "Invalid OKTA_DOMAIN: {}. It must begin with 'https://' and not end with a '/'. Example: 'https://beyondidentity.okta.com'",
                okta_domain
            ));
        }
    } else {
        return Err("OKTA_DOMAIN is missing. Please set OKTA_DOMAIN in your .env file. Example: 'OKTA_DOMAIN=https://beyondidentity.okta.com'".to_string());
    }

    // Validate BEYOND_IDENTITY_API_BASE_URL
    if let Some(api_base_url) = env_vars.get("BEYOND_IDENTITY_API_BASE_URL") {
        if !valid_api_urls.contains(&api_base_url.as_str()) {
            return Err(format!(
                "Invalid BEYOND_IDENTITY_API_BASE_URL: {}. It must be one of the following: 'https://api-us.beyondidentity.run', 'https://api-us.beyondidentity.xyz', 'https://api-us.beyondidentity.com'",
                api_base_url
            ));
        }
    } else {
        return Err("BEYOND_IDENTITY_API_BASE_URL is missing. Please set BEYOND_IDENTITY_API_BASE_URL in your .env file. Example: 'BEYOND_IDENTITY_API_BASE_URL=https://api-us.beyondidentity.run'".to_string());
    }

    // Validate BEYOND_IDENTITY_AUTH_BASE_URL
    if let Some(auth_base_url) = env_vars.get("BEYOND_IDENTITY_AUTH_BASE_URL") {
        if !valid_auth_urls.contains(&auth_base_url.as_str()) {
            return Err(format!(
                "Invalid BEYOND_IDENTITY_AUTH_BASE_URL: {}. It must be one of the following: 'https://auth-us.beyondidentity.run', 'https://auth-us.beyondidentity.xyz', 'https://auth-us.beyondidentity.com'",
                auth_base_url
            ));
        }
    } else {
        return Err("BEYOND_IDENTITY_AUTH_BASE_URL is missing. Please set BEYOND_IDENTITY_AUTH_BASE_URL in your .env file. Example: 'BEYOND_IDENTITY_AUTH_BASE_URL=https://auth-us.beyondidentity.run'".to_string());
    }

    // Validate ADMIN_PRIMARY_EMAIL_ADDRESS
    if let Some(admin_email) = env_vars.get("ADMIN_PRIMARY_EMAIL_ADDRESS") {
        if !email_regex.is_match(admin_email) {
            return Err(format!(
                "Invalid ADMIN_PRIMARY_EMAIL_ADDRESS: {}. It must be a valid email address. Example: 'admin@example.com'",
                admin_email
            ));
        }
    } else {
        return Err("ADMIN_PRIMARY_EMAIL_ADDRESS is missing. Please set ADMIN_PRIMARY_EMAIL_ADDRESS in your .env file. Example: 'ADMIN_PRIMARY_EMAIL_ADDRESS=admin@example.com'".to_string());
    }

    Ok(())
}
