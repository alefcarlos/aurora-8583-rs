use crate::domain::validations;
use aurora_8583::iso8583;
use aurora_8583::TryValidate;

pub fn execute(message: &iso8583::ISOMessage) -> Result<super::Result, super::Error> {
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
