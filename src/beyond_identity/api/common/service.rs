use super::api_client::ApiClient;

pub struct Service {
    pub api_client: ApiClient,
}

impl Service {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }
}
