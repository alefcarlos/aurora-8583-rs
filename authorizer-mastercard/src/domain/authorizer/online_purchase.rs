use crate::domain::{OnlinePurchaseTransaction, validations};
use aurora_8583::{TryValidate, TryAuthorizerTransaction};


impl TryAuthorizerTransaction<super::Result> for OnlinePurchaseTransaction {
    type Error = super::Error;
    fn try_authorize(&self) -> Result<super::Result, Self::Error>{
        let message = &self.0;

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
}