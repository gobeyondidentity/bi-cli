use http::Extensions;
use log::{debug, error};
use reqwest::{Body, Request, Response};
use reqwest_middleware::{Middleware, Next, Result};

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // Extract the body as bytes if present
        let body_bytes = if let Some(body) = req.body_mut().take() {
            match body.as_bytes() {
                Some(bytes) => bytes.to_vec(),
                None => {
                    error!("Failed to read request body");
                    Vec::new() // Default to an empty body in case of error
                }
            }
        } else {
            Vec::new() // Handle case where body is None
        };

        // Log the HTTP method, URL, and body
        debug!(
            "Sending request: method = {:?}, url = {}, body = {:?}",
            req.method(),
            req.url(),
            String::from_utf8_lossy(&body_bytes),
        );

        // Reconstruct the body and set it back to the request
        *req.body_mut() = Some(Body::from(body_bytes));

        // Proceed with the request
        let result = next.run(req, extensions).await;

        // Log the response status
        match &result {
            Ok(response) => {
                debug!("Received response: status = {}", response.status());
            }
            Err(err) => {
                error!("Request error: {:?}", err);
            }
        }

        result
    }
}
