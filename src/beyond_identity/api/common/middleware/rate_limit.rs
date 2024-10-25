use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error, Middleware, Next};

pub struct RespectRateLimitMiddleware;

impl RespectRateLimitMiddleware {
    pub fn new_client() -> ClientWithMiddleware {
        let client = Client::new();
        ClientBuilder::new(client).with(Self).build()
    }
}

#[async_trait::async_trait]
impl Middleware for RespectRateLimitMiddleware {
    async fn handle(
        &self,
        req: reqwest::Request,
        ext: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<reqwest::Response, Error> {
        let duplicate_request = req.try_clone().ok_or_else(|| {
            Error::Middleware(anyhow::anyhow!(
                "Request object is not clonable. Are you passing a streaming body?".to_string()
            ))
        })?;

        let response = next.clone().run(duplicate_request, ext).await?;

        let status = response.status();
        if let (reqwest::StatusCode::TOO_MANY_REQUESTS, Some(delay_secs)) =
            (status, response.headers().get(reqwest::header::RETRY_AFTER))
        {
            if let Some(delay_secs) = delay_secs.to_str().ok().and_then(|ds| ds.parse().ok()) {
                tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
                return next.run(req, ext).await;
            }
        }

        Ok(response)
    }
}
