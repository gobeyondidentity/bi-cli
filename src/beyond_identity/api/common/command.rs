use crate::common::error::BiError;
use serde::Serialize;

pub async fn serialize<T>(
    fut: impl std::future::Future<Output = Result<T, BiError>>,
) -> Result<String, BiError>
where
    T: Serialize,
{
    match fut.await {
        Ok(res) => {
            let json_value = serde_json::to_value(res).map_err(BiError::from)?;
            serde_json::to_string_pretty(&json_value).map_err(BiError::from)
        }
        Err(BiError::RequestError(status, body)) => {
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(parsed_json) => serde_json::to_string_pretty(&parsed_json).or(Ok(body)),
                Err(_) => Ok(format!("Error (HTTP {}): {}", status, body)),
            }
        }
        Err(e) => Err(e),
    }
}
