use std::{error, fmt::Display};

#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidTransaction,
    SaldoInsuficiente,
    CartaoInexistente,
    SenhaInvalida,
    InvalidCVC,
    InvalidCardExpirationDate,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidTransaction => write!(f, "InvalidTransaction(0100)"),
            Error::SaldoInsuficiente => write!(f, "SaldoInsuficiente(0400"),
            Error::CartaoInexistente => write!(f, "CartaoInexistente(0400"),
            Error::SenhaInvalida => write!(f, "SenhaInvalida(0400"),
            Error::InvalidCVC => write!(f, "InvalidCVC(0400"),
            Error::InvalidCardExpirationDate => write!(f, "InvalidCardExpirationDate(0400"),
        }
    }
}

impl error::Error for Error {}

pub struct ValidationSomeResult {}

pub enum ValidationResult {
    Ok,
    Some(ValidationSomeResult),
}

pub enum Result {
    Authorization(String, u32),
}
