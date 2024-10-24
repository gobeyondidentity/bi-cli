use super::error::BiError;
use ambassador::delegatable_trait;
use async_trait::async_trait;

#[async_trait]
#[delegatable_trait]
pub trait Executable {
    async fn execute(&self) -> anyhow::Result<(), BiError>;
}
