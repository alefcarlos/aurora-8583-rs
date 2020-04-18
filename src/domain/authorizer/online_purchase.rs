use crate::domain::{validations::validate_cvc, ISOMessage};
use super::execute::{AuthorizerError, AuthorizerResult};

pub fn process_online(message: &ISOMessage) -> Result<AuthorizerResult, AuthorizerError> {
    validate_cvc(message)?;

    //Não ocorreu erro, então retornar sucesso
    Ok(AuthorizerResult::Authorization("00123".to_string(), 2))
}