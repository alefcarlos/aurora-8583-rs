use crate::domain::{self, authorizer};

pub struct ValidateCVC {
    cvc: String,
}

impl domain::TryExecute<authorizer::ValidationResult, authorizer::Error> for ValidateCVC {
    fn execute(&self) -> Result<authorizer::ValidationResult, authorizer::Error> {
        Ok(authorizer::ValidationResult::Ok)
    }
}
