use crate::common::error::BiError;

use serde::Serialize;

pub async fn output<T>(
    fut: impl std::future::Future<Output = Result<T, BiError>>,
) -> Result<(), BiError>
where
    T: Serialize,
{
    match fut.await {
        Ok(res) => {
            let json_value = serde_json::to_value(res).map_err(BiError::from)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&json_value).map_err(BiError::from)?
            );
            Ok(())
        }
        Err(BiError::RequestError(status, body)) => {
            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(parsed_json) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&parsed_json).map_err(BiError::from)?
                    )
                }
                Err(_) => println!("{}", format!("Error (HTTP {}): {}", status, body)),
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
