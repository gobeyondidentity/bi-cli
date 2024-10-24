use crate::common::error::BiError;
use field_types::FieldName;
use std::fmt;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum FilterOperation {
    Eq,
}

#[derive(Debug, Clone, FieldName)]
pub struct Filter {
    pub filter: String,
}

impl Filter {
    pub fn new<F>(
        raw_filter: Option<String>,
        field_parser: fn(&str) -> Result<F, F::Err>,
    ) -> Result<Option<Self>, BiError>
    where
        F: fmt::Debug + Clone + ToString + FromStr,
        F::Err: fmt::Debug, // Adding this bound ensures we can use `{:?}` with the error type.
    {
        let raw_filter = match raw_filter {
            Some(filter) if !filter.trim().is_empty() => filter,
            _ => return Ok(None), // Return None if the filter is missing or empty
        };

        let parts: Vec<&str> = raw_filter.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(BiError::InvalidFilter(
                "Invalid filter format. Expected format: 'field operation value'".to_string(),
            ));
        }

        let field = field_parser(parts[0])
            .map_err(|e| BiError::InvalidFilter(format!("Invalid field: {}: {:?}", parts[0], e)))?;
        let operation = FilterOperation::from_str(parts[1])
            .map_err(|_| BiError::InvalidFilter(format!("Invalid operation: {}", parts[1])))?;
        let value = parts[2];

        let filter = format!("{} {} \"{}\"", &field.to_string(), operation, value).to_string();

        Ok(Some(Filter { filter }))
    }
}
