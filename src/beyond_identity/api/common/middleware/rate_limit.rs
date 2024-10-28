use log::debug;
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
        let mut retries = 0;
        let max_retries = 10;

        loop {
            let duplicate_request = req.try_clone().ok_or_else(|| {
                Error::Middleware(anyhow::anyhow!(
                    "Request object is not clonable. Are you passing a streaming body?".to_string()
                ))
            })?;

            let response = next.clone().run(duplicate_request, ext).await?;
            let status = response.status();

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                debug!("Received TOO_MANY_REQUESTS status code.");
                if let Some(delay_secs) = response
                    .headers()
                    .get(reqwest::header::RETRY_AFTER)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|s| s.parse().ok())
                {
                    debug!(
                        "Received RETRY_AFTER header. Retrying after {} seconds",
                        delay_secs
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
                } else if retries < max_retries {
                    let backoff_delay = 2u64.pow(retries).min(60);
                    debug!(
                        "Did not receive RETRY_AFTER header. Retrying after {} seconds",
                        backoff_delay
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(backoff_delay)).await;
                    retries += 1;
                } else {
                    return Err(Error::Middleware(anyhow::anyhow!(
                        "Max retries reached without a RETRY_AFTER header."
                    )));
                }
            } else {
                return Ok(response);
            }
        }
    }
}
