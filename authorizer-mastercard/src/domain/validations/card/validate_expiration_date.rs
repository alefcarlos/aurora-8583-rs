use crate::domain::authorizer;
pub struct ValidateExpiration {
    request_date: String,
    card_date: String,
}

impl ValidateExpiration {
    pub fn new(request_date: String, card_date: String) -> Self {
        Self {
            request_date,
            card_date,
        }
    }
}

impl aurora_8583::TryValidate<authorizer::ValidationResult> for ValidateExpiration {
    type Error = authorizer::Error;

    fn try_validate(&self) -> Result<authorizer::ValidationResult, Self::Error> {
        if self.request_date != self.card_date {
            return Err(authorizer::Error::InvalidCardExpirationDate);
        }

        Ok(authorizer::ValidationResult::Ok)
    }
}
