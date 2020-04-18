use crate::domain::TransactionType;
use super::online_purchase::process_online;

pub enum AuthorizerError {
    InvalidTransaction,
    SaldoInsuficiente,
    CartaoInexistente,
    SenhaInvalida,
}

pub enum AuthorizerResult {
    Authorization(String, u32),
}

pub fn execute_auth_flow(trx: TransactionType) -> Result<AuthorizerResult, AuthorizerError> {
    let result = match trx {
        TransactionType::OlinePurchase(message) => process_online(&message),
        _ => Err(AuthorizerError::InvalidTransaction)
    };

    match result {
        Ok(v) => Ok(v),
        Err(v) => Err(v)
    }
}