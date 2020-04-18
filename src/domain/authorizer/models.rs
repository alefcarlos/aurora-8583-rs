pub enum Error {
    InvalidTransaction,
    SaldoInsuficiente,
    CartaoInexistente,
    SenhaInvalida,
}

pub enum ValidationResult{
    Ok,
}

pub enum Result {
    Authorization(String, u32),
}