use crate::domain::{self, authorizer};

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

impl domain::validations::TryValidate<authorizer::ValidationResult, authorizer::Error> for ValidateExpiration {
    fn try_validate(&self) -> Result<authorizer::ValidationResult, authorizer::Error> {
        if self.request_date != self.card_date {
            return Err(authorizer::Error::InvalidCardExpirationDate);
        }

        Ok(authorizer::ValidationResult::Ok)
    }
}
