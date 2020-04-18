use crate::domain::{self, authorizer};

pub struct ValidateCVC {
    cvc: String,
}

impl domain::TryExecute<authorizer::ValidationResult, authorizer::Error> for ValidateCVC {
    fn execute(&self) -> Result<authorizer::ValidationResult, authorizer::Error> {
        match self.cvc.as_str() {
            "123" => Ok(authorizer::ValidationResult::Ok),
            _ => Err(authorizer::Error::InvalidCVC),
        }
    }
}
