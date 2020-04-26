use aurora_8583::{iso8583::ISOMessage, Unvalidated, ValidateResult, Validated};

pub fn validate_expiration(iso_request: &ISOMessage) -> ValidateResult {
    // TODO: modeling this
    let request_date = "";

    if request_date != iso_request.card.expiration_date {
        return Err(Unvalidated("Expired...".to_owned()));
    }

    Ok(Validated)
}

pub struct ValidateExpiration {
    pub request_date: String,
    pub card_date: String,
}

impl ValidateExpiration {
    pub fn new(request_date: String, card_date: String) -> Self {
        Self {
            request_date,
            card_date,
        }
    }
}
