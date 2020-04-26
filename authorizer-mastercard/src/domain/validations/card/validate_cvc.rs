use aurora_8583::iso8583;
use aurora_8583::{Unvalidated, ValidateResult, Validated};
use iso8583::ISOMessage;

pub fn validate_cvc(iso_request: &ISOMessage) -> ValidateResult {
    match iso_request.card.sequence.as_str() {
        "123" => Ok(Validated),
        _ => Err(Unvalidated("Invalid CVC".to_owned())),
    }
}
