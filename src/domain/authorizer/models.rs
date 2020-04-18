pub enum Error {
    InvalidTransaction,
    SaldoInsuficiente,
    CartaoInexistente,
    SenhaInvalida,
    InvalidCVC,
    InvalidCardExpirationDate,
}

pub struct ValidationSomeResult {}

pub enum ValidationResult {
    Ok,
    Some(ValidationSomeResult),
}

pub enum Result {
    Authorization(String, u32),
}
