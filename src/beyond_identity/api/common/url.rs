use crate::common::error::BiError;
use crate::setup::tenants::tenant::TenantConfig;

use url::Url;

pub struct URLBuilder {
    url: Option<Url>,
    api_base_url: String,
    auth_base_url: String,
    tenant_id: String,
    realm_id: String,
}

impl URLBuilder {
    /// Initializes the URLBuilder with the tenant configuration.
    pub fn build(tenant_config: &TenantConfig) -> Self {
        URLBuilder {
            url: None,
            api_base_url: tenant_config.api_base_url.clone(),
            auth_base_url: tenant_config.auth_base_url.clone(),
            tenant_id: tenant_config.tenant_id.clone(),
            realm_id: tenant_config.realm_id.clone(),
        }
    }

    /// Specifies that this is an API URL.
    pub fn api(mut self) -> Self {
        self.url = Some(Url::parse(&self.api_base_url).expect("Invalid API base URL"));
        self
    }

    /// Specifies that this is an Auth URL.
    pub fn auth(mut self) -> Self {
        self.url = Some(Url::parse(&self.auth_base_url).expect("Invalid Auth base URL"));
        self
    }

    /// Helper method to get a mutable reference to the URL.
    fn url_mut(&mut self) -> &mut Url {
        self.url
            .as_mut()
            .expect("URL not initialized. Call api() or auth() first.")
    }

    /// Adds the tenant ID to the URL path.
    pub fn add_tenant(mut self) -> Self {
        let tenant_id = self.tenant_id.clone();
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(&["v1", "tenants", &tenant_id]);
        self
    }

    /// Adds the realm ID to the URL path.
    pub fn add_realm(mut self) -> Self {
        let realm_id = self.realm_id.clone();
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(&["realms", &realm_id]);
        self
    }

    /// Adds the specified realm ID to the URL path.
    pub fn add_realm_with_override(mut self, id: String) -> Self {
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(&["realms", &id]);
        self
    }

    /// Adds additional path segments to the URL.
    pub fn add_path(mut self, segments: Vec<&str>) -> Self {
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .extend(segments.iter().copied());
        self
    }

    /// Appends a colon suffix to the last path segment.
    pub fn add_custom_method(mut self, suffix: &str) -> Self {
        // Get the current path segments as a vector of strings.
        let mut segments: Vec<String> = self
            .url_mut()
            .path_segments()
            .map(|segments| segments.map(|s| s.to_string()).collect())
            .unwrap_or_default();

        if let Some(last_segment) = segments.last_mut() {
            // Append the colon suffix to the last segment.
            *last_segment = format!("{}:{}", last_segment, suffix);
        } else {
            // If there are no segments, add the suffix as a new segment.
            segments.push(format!(":{}", suffix));
        }

        // Clear existing path segments and set the modified ones.
        self.url_mut()
            .path_segments_mut()
            .expect("Cannot be base")
            .clear()
            .extend(segments.iter().map(|s| &**s));

        self
    }

    /// Adds a query parameter to the URL if the value is `Some`.
    pub fn add_query_param(mut self, key: &str, value: Option<&str>) -> Self {
        if let Some(value) = value {
            self.url_mut().query_pairs_mut().append_pair(key, value);
        }
        self
    }

    /// Converts the URLBuilder into a `String` representing the final URL.
    pub fn to_string(self) -> Result<String, BiError> {
        self.url
            .ok_or_else(|| {
                BiError::StringError("URL not initialized. Call api() or auth() first.".into())
            })
            .map(|url| url.to_string())
    }
}
