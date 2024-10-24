use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        log::debug!(
            "Sending request: method = {:?}, url = {}, body = {:?}",
            req.method(),
            req.url(),
            req.body().map(|b| format!("{:?}", b))
        );

        let result = next.run(req, extensions).await;

        match &result {
            Ok(response) => {
                log::debug!("Received response: status = {}", response.status());
            }
            Err(err) => {
                log::error!("Request error: {:?}", err);
            }
        }

        result
    }
}
