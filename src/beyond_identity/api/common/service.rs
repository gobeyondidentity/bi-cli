use super::api_client::ApiClient;

pub struct Service {
    pub api_client: ApiClient,
}

impl Service {
    pub async fn new() -> Self {
        Self {
            api_client: ApiClient::new().await,
        }
    }
}
