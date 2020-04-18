use crate::domain::{self, authorizer};
use domain::ISOMessage;
pub struct ValidateCVC {
    cvc: String,
}

impl domain::validations::TryValidate<authorizer::ValidationResult, authorizer::Error> for ValidateCVC {
    fn try_validate(&self) -> Result<authorizer::ValidationResult, authorizer::Error> {
        match self.cvc.as_str() {
            "123" => Ok(authorizer::ValidationResult::Ok),
            _ => Err(authorizer::Error::InvalidCVC),
        }
    }
}

impl From<&ISOMessage> for ValidateCVC {
    fn from(_: &ISOMessage) -> Self {
        Self {
            cvc: "123".to_string(),
        }
    }
}
