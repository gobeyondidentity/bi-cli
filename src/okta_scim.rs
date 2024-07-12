use crate::config::Config;
use crate::{bi_scim::BiScimConfig, error::BiError, tenant::TenantConfig};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
pub struct OktaAppResponse {
    pub id: String,
    pub name: String,
    pub label: String,
    pub status: String,
    pub lastUpdated: String,
    pub created: String,
    pub accessibility: Accessibility,
    pub visibility: Visibility,
    pub features: Vec<String>,
    pub signOnMode: String,
    pub credentials: Credentials,
    pub settings: Settings,
    pub _links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessibility {
    pub selfService: bool,
    pub errorRedirectUrl: Option<String>,
    pub loginRedirectUrl: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Visibility {
    pub autoLaunch: bool,
    pub autoSubmitToolbar: bool,
    pub hide: Hide,
    pub appLinks: AppLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hide {
    pub iOS: bool,
    pub web: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppLinks {
    pub scim2testapp_login: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub userNameTemplate: UserNameTemplate,
    pub signing: Signing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNameTemplate {
    pub template: String,
    #[serde(rename = "type")]
    pub template_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signing {
    pub kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub notifications: Notifications,
    pub manualProvisioning: bool,
    pub implicitAssignment: bool,
    pub notes: Notes,
    pub signOn: SignOn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub acsUrl: Option<String>,
    pub audienceUri: Option<String>,
    pub swaLoginUrl: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub vpn: Vpn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vpn {
    pub network: Network,
    pub message: Option<String>,
    pub helpUrl: Option<String>,
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
pub struct SignOn {
    pub defaultRelayState: Option<String>,
    pub ssoAcsUrlOverride: Option<String>,
    pub audienceOverride: Option<String>,
    pub recipientOverride: Option<String>,
    pub destinationOverride: Option<String>,
    pub honorForceAuthn: bool,
    pub attributeStatements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub help: Link,
    pub metadata: Link,
    pub uploadLogo: Link,
    pub appLinks: Vec<AppLink>,
    pub profileEnrollment: Link,
    pub policies: Link,
    pub groups: Link,
    pub logo: Vec<Logo>,
    pub accessPolicy: Link,
    pub users: Link,
    pub deactivate: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    #[serde(rename = "type")]
    pub link_type: Option<String>,
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
    #[serde(rename = "type")]
    pub app_link_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logo {
    pub name: String,
    pub href: String,
    #[serde(rename = "type")]
    pub logo_type: String,
}

async fn create_scim_app(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_scim_config: &BiScimConfig,
) -> Result<OktaAppResponse, BiError> {
    let okta_base_url = config.okta_domain.clone();
    let okta_api_key = config.okta_api_key.clone();
    let bi_api_base_url = config.beyond_identity_api_base_url.clone();
    let tenant_id = tenant_config.tenant_id.clone();
    let realm_id = tenant_config.realm_id.clone();

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
        .post(format!("{}/api/v1/apps", okta_base_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

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
//         .put(format!("{}/api/v1/apps/{}", okta_base_url, app_id))
//         .header("Content-Type", "application/json")
//         .header("Authorization", format!("SSWS {}", okta_api_key))
//         .json(&payload)
//         .send()
//         .await?;

//     let status = response.status();
//     let response_text = response.text().await?;
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

    let response = client
        .put(format!(
            "{}/api/v1/apps/{}/groups/{}",
            okta_base_url, app_id, group.id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("SSWS {}", okta_api_key))
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;
    println!("Response status: {}", status);
    println!("Response body: {}", response_text);

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
        log::debug!("Assigning group: {:?}", group);
        assign_group_to_app(client, config, app_id, &group).await?;
        let sleep_duration = rand::thread_rng().gen_range(5..=10);
        println!("Sleeping for {} seconds...", sleep_duration);
        sleep(Duration::from_secs(sleep_duration)).await;
    }
    Ok(())
}

pub async fn create_scim_app_in_okta(
    client: &Client,
    config: &Config,
    tenant_config: &TenantConfig,
    bi_scim_config: &BiScimConfig,
) -> OktaAppResponse {
    let config_path = config.file_paths.okta_scim_app_config.clone();
    if Path::new(&config_path).exists() {
        let data = fs::read_to_string(config_path).expect("Unable to read file");
        serde_json::from_str(&data).expect("JSON was not well-formatted")
    } else {
        let response = create_scim_app(client, config, tenant_config, bi_scim_config)
            .await
            .expect("Failed to create okta scim app");
        let serialized = serde_json::to_string_pretty(&response)
            .expect("Failed to serialize okta scim app response");
        fs::write(config_path, serialized).expect("Unable to write file");
        response
    }
}
