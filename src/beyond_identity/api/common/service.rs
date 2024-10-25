use super::api_client::ApiClient;

pub struct Service {
    pub api_client: ApiClient,
}

impl Service {
    pub fn new() -> Self {
        Self {
            api_client: ApiClient::new(),
        }
    }
}
