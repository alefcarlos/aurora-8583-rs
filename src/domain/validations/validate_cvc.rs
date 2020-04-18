use super::ValidationResult;
use crate::domain::{ISOMessage, AuthorizerError};

pub fn validate_cvc(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
    Ok(ValidationResult::None)
}
