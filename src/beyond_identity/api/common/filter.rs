use crate::common::error::BiError;
use scim_filter::parser::filter;

#[derive(Debug, Clone)]
pub struct Filter(pub String);

impl Filter {
    pub fn new(maybe_filter: Option<String>) -> Result<Option<Self>, BiError> {
        let f = match maybe_filter {
            Some(filter) if !filter.trim().is_empty() => filter,
            _ => return Ok(None),
        };
        match filter(&f) {
            Ok(_) => Ok(Some(Filter(f))),
            Err(e) => Err(BiError::InvalidFilter(e.to_string())),
        }
    }
}
