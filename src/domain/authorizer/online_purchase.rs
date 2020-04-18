// use super::execute::{Error, Result};
// use crate::domain::{
//     validations::{validate_card_cvc, validate_card_expiration_date, ValidationResult},
//     ISOMessage,
// };

// pub fn execute_online_purchase(message: &ISOMessage) -> Result<Result, Error> {
//     validate_card(&message)?;

//     //Não ocorreu erro, então retornar sucesso
//     Ok(Result::Authorization("00123".to_string(), 2))
// }

// fn validate_card(message: &ISOMessage) -> Result<ValidationResult, Error> {
//     validate_card_cvc(message)?;

//     validate_card_expiration_date(message)
// }
use crate::domain::{TransactionType, TryExecute};

impl TryExecute<super::Result, super::Error>
    for TransactionType
{
    fn execute(
        &self,
    ) -> Result<super::Result, super::Error> {
        todo!()
    }
}
