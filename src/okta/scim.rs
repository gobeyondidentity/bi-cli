use crate::common::config::Config;
use crate::common::error::BiError;
use rand::Rng;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OktaAppResponse {
    pub id: String,
    pub name: String,
    pub label: String,
    pub status: String,
    pub last_updated: String,
    pub created: String,
    pub accessibility: Accessibility,
    pub visibility: Visibility,
    pub features: Vec<String>,
    pub sign_on_mode: String,
    pub credentials: Credentials,
    pub settings: Settings,
    #[serde(rename = "_links")]
    pub _links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility {
    pub self_service: bool,
    pub error_redirect_url: Option<String>,
    pub login_redirect_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    pub auto_launch: bool,
    pub auto_submit_toolbar: bool,
    pub hide: Hide,
    pub app_links: AppLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hide {
    #[serde(rename = "iOS")]
    pub ios: bool,
    pub web: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLinks {
    pub scim2testapp_login: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub user_name_template: UserNameTemplate,
    pub signing: Signing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNameTemplate {
    pub template: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signing {
    pub kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub app: AppSettings,
    pub notifications: Notifications,
    pub manual_provisioning: bool,
    pub implicit_assignment: bool,
    pub notes: Notes,
    pub sign_on: SignOn,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub acs_url: Option<String>,
    pub audience_uri: Option<String>,
    pub swa_login_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub vpn: Vpn,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vpn {
    pub network: Network,
    pub message: Option<String>,
    pub help_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub connection: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub admin: Option<String>,
    pub enduser: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOn {
    pub default_relay_state: Option<String>,
    pub sso_acs_url_override: Option<String>,
    pub audience_override: Option<String>,
    pub recipient_override: Option<String>,
    pub destination_override: Option<String>,
    pub honor_force_authn: bool,
    pub attribute_statements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub help: Link,
    pub metadata: Link,
    pub upload_logo: Link,
    pub app_links: Vec<AppLink>,
    pub profile_enrollment: Link,
    pub policies: Link,
    pub groups: Link,
    pub logo: Vec<Logo>,
    pub access_policy: Link,
    pub users: Link,
    pub deactivate: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub r#type: Option<String>,
    pub hints: Option<Hints>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hints {
    pub allow: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppLink {
    pub name: String,
    pub href: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logo {
    pub name: String,
    pub href: String,
    pub r#type: String,
}

async fn create_scim_app(client: &Client, config: &Config) -> Result<OktaAppResponse, BiError> {
    let okta_base_url = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();

    let url = format!("{}/api/v1/apps", okta_base_url);

    let payload = json!({
        "name": "scim2testapp",
        "label": "Beyond Identity SCIM",
        "status": "ACTIVE",
        "signOnMode": "SAML_2_0",
        "accessibility": {
            "selfService": false,
            "errorRedirectUrl": null,
            "loginRedirectUrl": null
        },
        "visibility": {
            "autoLaunch": false,
            "autoSubmitToolbar": true,
            "hide": {
                "iOS": false,
                "web": false
            },
            "appLinks": {
                "scim2testapp_login": true
            }
        },
        "features": [
            "IMPORT_PROFILE_UPDATES",
            "PUSH_NEW_USERS",
            "PUSH_USER_DEACTIVATION",
            "SCIM_PROVISIONING",
            "GROUP_PUSH",
            "REACTIVATE_USERS",
            "IMPORT_NEW_USERS"
        ],
        "credentials": {
            "userNameTemplate": {
                "template": "${source.login}",
                "type": "BUILT_IN"
            },
        },
        "settings": {
            "app": {
                "acsUrl": null,
                "audienceUri": null,
                "swaLoginUrl": null
            },
            "notifications": {
                "vpn": {
                    "network": {
                        "connection": "DISABLED"
                    },
                    "message": null,
                    "helpUrl": null
                }
            },
            "manualProvisioning": false,
            "implicitAssignment": false,
            "notes": {
                "admin": null,
                "enduser": null
            },
            "signOn": {
                "defaultRelayState": null,
                "ssoAcsUrlOverride": null,
                "audienceOverride": null,
                "recipientOverride": null,
                "destinationOverride": null,
                "honorForceAuthn": false,
                "attributeStatements": []
            }
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    let app_response: OktaAppResponse = serde_json::from_str(&response_text)?;

    // enable_provisioning_in_okta(
    //     client,
    //     config,
    //     tenant_config,
    //     bi_scim_config,
    //     &app_response.id,
    // )
    // .await?;

    assign_all_groups_to_app(client, config, &app_response.id).await?;

    Ok(app_response)
}

// async fn enable_provisioning_in_okta(
//     client: &Client,
//     config: &Config,
//     tenant_config: &TenantConfig,
//     bi_scim_config: &BiScimConfig,
//     app_id: &str,
// ) -> Result<(), BiError> {
//     let okta_base_url = config.okta_domain.clone();
//     let okta_api_key = config.okta_api_key.clone();
//     let bi_api_base_url = config.beyond_identity_api_base_url.clone();
//     let tenant_id = tenant_config.tenant_id.clone();
//     let realm_id = tenant_config.realm_id.clone();
//     let scim_base_url = format!(
//         "{}/v1/tenants/{}/realms/{}/scim/v2",
//         bi_api_base_url, tenant_id, realm_id
//     );
//     let scim_bearer_token = bi_scim_config.oauth_bearer_token.clone();

//     let url = format!("{}/api/v1/apps/{}", okta_base_url, app_id);

//     let payload = json!({
//         "status": "ACTIVE",
//         "features": [
//             "IMPORT_PROFILE_UPDATES",
//             "PUSH_NEW_USERS",
//             "PUSH_USER_DEACTIVATION",
//             "SCIM_PROVISIONING",
//             "GROUP_PUSH",
//             "REACTIVATE_USERS",
//             "IMPORT_NEW_USERS"
//         ],
//         "settings": {
//             "app": {
//                 "acsUrl": null,
//                 "audienceUri": null,
//                 "swaLoginUrl": null
//             },
//             "notifications": {
//                 "vpn": {
//                     "network": {
//                         "connection": "DISABLED"
//                     },
//                     "message": null,
//                     "helpUrl": null
//                 }
//             },
//             "manualProvisioning": false,
//             "implicitAssignment": false,
//             "notes": {
//                 "admin": null,
//                 "enduser": null
//             },
//             "signOn": {
//                 "defaultRelayState": null,
//                 "ssoAcsUrlOverride": null,
//                 "audienceOverride": null,
//                 "recipientOverride": null,
//                 "destinationOverride": null,
//                 "honorForceAuthn": false,
//                 "attributeStatements": []
//             },
//             "provisioning": {
//                 "enabled": true,
//                 "profileMaster": true,
//                 "groupAssignments": true,
//                 "groupPush": {
//                     "enabled": true,
//                     "maxBufferSize": 100
//                 },
//                 "conditions": {
//                     "deprovisioned": {
//                         "action": "NONE"
//                     },
//                     "suspended": {
//                         "action": "NONE"
//                     }
//                 },
//                 "scim": {
//                     "baseUrl": scim_base_url,
//                     "authType": "OAUTH_BEARER_TOKEN",
//                     "authToken": scim_bearer_token
//                 }
//             }
//         }
//     });

//     let response = client
//         .put(&url)
//         .header("Content-Type", "application/json")
//         .header("Authorization", format!("SSWS {}", okta_api_key))
//         .json(&payload)
//         .send()
//         .await?;

//     let status = response.status();
//     let response_text = response.text().await?;
//     log::debug!(
//         "{} response status: {} and text: {}",
//         url,
//         status,
//         response_text
//     );
//     println!("Response status: {}", status);
//     println!("Response body: {}", response_text);

//     if !status.is_success() {
//         return Err(BiError::RequestError(status, response_text));
//     }

//     Ok(())
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct OktaGroup {
    id: String,
    profile: OktaGroupProfile,
}

#[derive(Debug, Serialize, Deserialize)]
struct OktaGroupProfile {
    name: String,
    description: Option<String>,
}

pub async fn list_all_okta_groups(
    client: &Client,
    config: &Config,
) -> Result<Vec<OktaGroup>, BiError> {
    let okta_domain = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();
    let mut groups: Vec<OktaGroup> = Vec::new();
    let mut next_link: Option<String> = None;

    loop {
        let url = match next_link {
            Some(ref link) => link.clone(),
            None => format!("{}/api/v1/groups?limit=200", okta_domain),
        };

        let response = client
            .get(&url)
            .header("Authorization", format!("SSWS {}", okta_api_key))
            .send()
            .await?;

        let status = response.status();

        // Extract the Link header before consuming the response body
        next_link = response.headers().get("link").and_then(|link| {
            let link_str = link.to_str().ok()?;
            if link_str.contains("rel=\"next\"") {
                Some(link_str.split(';').next()?.trim().to_string())
            } else {
                None
            }
        });

        let response_text = response.text().await?;

        log::debug!(
            "{} response status: {} and text: {}",
            url,
            status,
            response_text
        );

        if !status.is_success() {
            return Err(BiError::RequestError(status, response_text));
        }

        let mut group_list: Vec<OktaGroup> = serde_json::from_str(&response_text)?;

        // Filter out "Okta Administrators" group
        group_list.retain(|group| group.profile.name != "Okta Administrators");

        groups.extend(group_list);

        if next_link.is_none() {
            break;
        }
    }

    Ok(groups)
}

async fn assign_group_to_app(
    client: &Client,
    config: &Config,
    app_id: &str,
    group: &OktaGroup,
) -> Result<(), BiError> {
    let okta_base_url = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();

    let url = format!(
        "{}/api/v1/apps/{}/groups/{}",
        okta_base_url, app_id, group.id
    );

    let response = client
        .put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    log::debug!(
        "{} response status: {} and text: {}",
        url,
        status,
        response_text
    );

    if !status.is_success() {
        return Err(BiError::RequestError(status, response_text));
    }

    Ok(())
}

pub async fn assign_all_groups_to_app(
    client: &Client,
    config: &Config,
    app_id: &str,
) -> Result<(), BiError> {
    let groups = list_all_okta_groups(client, config).await?;
    for group in groups {
        log::info!("Assigning group: {:?}", group);
        assign_group_to_app(client, config, app_id, &group).await?;
        let sleep_duration = rand::thread_rng().gen_range(5..=10);
        println!("Sleeping for {} seconds...", sleep_duration);
        sleep(Duration::from_secs(sleep_duration)).await;
    }
    Ok(())
}

pub async fn load_scim_app_in_okta(config: &Config) -> Result<OktaAppResponse, BiError> {
    let config_path = config.file_paths.okta_scim_app_config.clone();
    let data = fs::read_to_string(&config_path)
        .map_err(|_| BiError::ConfigFileNotFound(config_path.clone()))?;
    let okta_app_response: OktaAppResponse =
        serde_json::from_str(&data).map_err(BiError::SerdeError)?;
    Ok(okta_app_response)
}

pub async fn create_scim_app_in_okta(
    client: &Client,
    config: &Config,
) -> Result<OktaAppResponse, BiError> {
    let response = create_scim_app(client, config).await?;
    let serialized = serde_json::to_string_pretty(&response)?;

    let config_path = config.file_paths.okta_scim_app_config.clone();
    fs::write(config_path.clone(), serialized)
        .map_err(|_| BiError::UnableToWriteFile(config_path))?;

    Ok(response)
}
