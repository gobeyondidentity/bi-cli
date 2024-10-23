use super::api::{IdentitiesApi, IdentityService};
use super::types::{Identities, IdentityFilterField};
use crate::beyond_identity::api::common::filter::Filter;
use crate::common::error::BiError;
use clap::Args;
use std::str::FromStr;

// ===============================
// Command
// ===============================

#[derive(Args, Debug, Clone)]
pub struct List {
    #[clap(long)]
    pub filter: Option<String>,
}

impl List {
    pub async fn execute(self, service: &IdentityService) -> Result<Identities, BiError> {
        service
            .list_identities(Filter::new(self.filter, IdentityFilterField::from_str)?)
            .await
    }
}
