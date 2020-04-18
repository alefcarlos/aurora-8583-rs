pub enum Error {
    InvalidTransaction,
    SaldoInsuficiente,
    CartaoInexistente,
    SenhaInvalida,
    InvalidCVC,
}

pub enum ValidationResult{
    Ok,
}

pub enum Result {
    Authorization(String, u32),
}