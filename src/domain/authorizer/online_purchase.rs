use super::execute::{AuthorizerError, AuthorizerResult};
use crate::domain::{
    validations::{validate_card_cvc, validate_card_expiration_date, ValidationResult},
    ISOMessage,
};

pub fn execute_online_purchase(message: &ISOMessage) -> Result<AuthorizerResult, AuthorizerError> {
    validate_card(&message)?;

    //Não ocorreu erro, então retornar sucesso
    Ok(AuthorizerResult::Authorization("00123".to_string(), 2))
}

fn validate_card(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
    validate_card_cvc(message)?;

    validate_card_expiration_date(message)
}
