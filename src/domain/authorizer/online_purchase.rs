use crate::{domain::validations, authorization_iso_8583::{TryValidate, iso_8583}};

pub fn execute(message: &iso_8583::ISOMessage) -> Result<super::Result, super::Error> {
    validations::card::ValidateCVC::from(message).try_validate()?;

    //Obter data de exp do cartão do banco
    validations::card::ValidateExpiration::new(
        message.card.expiration_date.clone(),
        "2416".to_string(),
    )
    .try_validate()?;

    //Não ocorreu erro, então retornar sucesso
    Ok(super::Result::Authorization("00123".to_string(), 2))
}
