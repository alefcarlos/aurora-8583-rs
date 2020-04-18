use super::execute::{AuthorizerError, AuthorizerResult};
use crate::domain::{validations::{validate_card_cvc, validate_card_expiration_date}, ISOMessage};

pub fn process_online(message: &ISOMessage) -> Result<AuthorizerResult, AuthorizerError> {
    validate_card_cvc(message)?;
    validate_card_expiration_date(message)?;

    //Não ocorreu erro, então retornar sucesso
    Ok(AuthorizerResult::Authorization("00123".to_string(), 2))
}