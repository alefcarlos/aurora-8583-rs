use crate::domain::{ISOMessage, AuthorizerError, validations::ValidationResult};

pub fn validate_card_cvc(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
    Ok(ValidationResult::None)
}
