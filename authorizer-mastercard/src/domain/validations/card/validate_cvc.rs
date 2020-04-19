use crate::domain::authorizer;
use aurora_8583::iso8583;

pub struct ValidateCVC {
    cvc: String,
}

impl aurora_8583::TryValidate<authorizer::ValidationResult> for ValidateCVC {
    type Error = authorizer::Error;
    
    fn try_validate(&self) -> Result<authorizer::ValidationResult, Self::Error> {
        match self.cvc.as_str() {
            "123" => Ok(authorizer::ValidationResult::Ok),
            _ => Err(authorizer::Error::InvalidCVC),
        }
    }
}

impl From<&iso8583::ISOMessage> for ValidateCVC {
    fn from(_: &iso8583::ISOMessage) -> Self {
        Self {
            cvc: "123".to_string(),
        }
    }
}

