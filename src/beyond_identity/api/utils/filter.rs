use std::fmt;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;
use urlencoding::encode;

use crate::common::error::BiError;

pub trait FilterField: fmt::Debug + Clone + ToString {}

#[derive(Debug, Clone, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum FilterOperation {
    Eq,
}

#[derive(Debug, Clone)]
pub struct Filter<F: FilterField> {
    field: F,
    operation: FilterOperation,
    value: String,
}

impl<F: FilterField> Filter<F> {
    pub fn to_string(&self) -> String {
        encode(&format!(
            "{} {} \"{}\"",
            self.field.to_string(),
            self.operation.to_string(),
            self.value
        ))
        .to_string()
    }

    /// Parses a filter given a field, operation, and value.
    fn parse(field: F, operation_str: &str, value: &str) -> Result<Self, String> {
        let operation = FilterOperation::from_str(operation_str)
            .map_err(|_| format!("Invalid operation: {}", operation_str))?;

        Ok(Filter {
            field,
            operation,
            value: value.to_string(),
        })
    }

    /// Parses a filter string with a custom field parser provided by the API.
    pub fn parse_with_field_parser(
        filter_str: Option<&str>,
        field_parser: fn(&str) -> Option<F>,
    ) -> Result<Option<Self>, BiError> {
        match filter_str {
            Some(filter) => {
                let parts: Vec<&str> = filter.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(BiError::InvalidFilter(
                        "Invalid filter format. Expected format: 'field operation value'"
                            .to_string(),
                    ));
                }

                let field = field_parser(parts[0]).ok_or_else(|| {
                    BiError::InvalidFilter(format!("Invalid field: {}", parts[0]))
                })?;

                Self::parse(field, parts[1], parts[2])
                    .map(Some)
                    .map_err(|e| BiError::InvalidFilter(e))
            }
            None => Ok(None),
        }
    }
}
