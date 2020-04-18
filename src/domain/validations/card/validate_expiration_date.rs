use crate::domain::{ISOMessage, AuthorizerError, validations::ValidationResult};

pub fn validate_card_expiration_date(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
    Ok(ValidationResult::None)
}
