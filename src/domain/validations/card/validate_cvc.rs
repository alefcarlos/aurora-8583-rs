use crate::{domain::authorizer, authorization_iso_8583};
use authorization_iso_8583::iso_8583;

pub struct ValidateCVC {
    cvc: String,
}

impl authorization_iso_8583::TryValidate<authorizer::ValidationResult, authorizer::Error> for ValidateCVC {
    fn try_validate(&self) -> Result<authorizer::ValidationResult, authorizer::Error> {
        match self.cvc.as_str() {
            "123" => Ok(authorizer::ValidationResult::Ok),
            _ => Err(authorizer::Error::InvalidCVC),
        }
    }
}

impl From<&iso_8583::ISOMessage> for ValidateCVC {
    fn from(_: &iso_8583::ISOMessage) -> Self {
        Self {
            cvc: "123".to_string(),
        }
    }
}
