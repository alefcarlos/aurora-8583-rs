use aurora_8583::{Authorized, Unauthorized};
use std::{error, fmt::Display};

#[derive(PartialEq, Debug)]
pub enum MyError {
    InvalidTransaction,
    SaldoInsuficiente,
    CartaoInexistente,
    SenhaInvalida,
    InvalidCVC,
    InvalidCardExpirationDate,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::InvalidTransaction => write!(f, "InvalidTransaction(0100)"),
            MyError::SaldoInsuficiente => write!(f, "SaldoInsuficiente(0400"),
            MyError::CartaoInexistente => write!(f, "CartaoInexistente(0400"),
            MyError::SenhaInvalida => write!(f, "SenhaInvalida(0400"),
            MyError::InvalidCVC => write!(f, "InvalidCVC(0400"),
            MyError::InvalidCardExpirationDate => write!(f, "InvalidCardExpirationDate(0400"),
        }
    }
}

impl error::Error for MyError {}

pub struct ValidationSomeResult {}

pub enum ValidationResult {
    Ok,
    Some(ValidationSomeResult),
}

pub enum MyResult {
    Authorization(String, u32),
}

impl From<Authorized> for MyResult {
    fn from(_: Authorized) -> Self {
        // TODO: modeling this
        MyResult::Authorization("Autorizado".to_owned(), 201)
    }
}

impl From<Unauthorized> for MyError {
    fn from(error: Unauthorized) -> Self {
        let message = error.0;

        match message.as_ref() {
            "Expired..." => Self::InvalidCVC,
            "Invalid CVC" => Self::InvalidCardExpirationDate,
            _ => Self::InvalidTransaction,
        }
    }
}
