use super::api_client::ApiClient;

use crate::common::database::models::Realm;
use crate::common::database::models::Tenant;

use paste::paste;

macro_rules! create_service_with_builder {
    ($service_name:ident) => {
        paste! {
            pub struct $service_name {
                pub api_client: ApiClient,
            }

            pub struct [<$service_name Builder>] {
                tenant: Option<Tenant>,
                realm: Option<Realm>,
            }

            impl $service_name {
                pub fn new() -> [<$service_name Builder>] {
                    [<$service_name Builder>] {
                        tenant: None,
                        realm: None,
                    }
                }
            }

            impl [<$service_name Builder>] {
                #[allow(dead_code)]
                pub fn tenant(mut self, tenant: Tenant) -> [<$service_name Builder>] {
                    self.tenant = Some(tenant);
                    self
                }

                #[allow(dead_code)]
                pub fn realm(mut self, realm: Realm) -> [<$service_name Builder>] {
                    self.realm = Some(realm);
                    self
                }

                pub async fn build(self) -> $service_name {
                    $service_name {
                        api_client: ApiClient::new(self.tenant, self.realm).await,
                    }
                }
            }
        }
    };
}

create_service_with_builder!(TenantsService);
create_service_with_builder!(RealmsService);
create_service_with_builder!(GroupsService);
create_service_with_builder!(IdentitiesService);
create_service_with_builder!(CredentialsService);
create_service_with_builder!(CredentialBindingJobsService);
