use super::Field;
use std::{collections::HashMap, convert::TryFrom};
use aurora_8583::iso8583;

const REQUIRED_DE_0100: &str = "0|2|3|22";
const REQUIRED_DE_0400: &str = "0|2|3|22";

#[derive(Clone)]
pub struct ISORequest {
    pub fields: Vec<Field>,
}

impl ISORequest {
    pub fn new(fields: Vec<Field>) -> Self {
        ISORequest { fields }
    }
}

impl ISORequest {
    ///Gets value from DE
    pub fn get_info(&self, id: &str) -> Option<String> {
        let item = self.fields.iter().find(|&field| field.id == id);

        match item {
            Some(x) => Some(x.value.clone()),
            None => None,
        }
    }

    ///Gets value from DE
    pub fn get_evaluated_info(&self, id: &str) -> String {
        let value = self.get_info(id);
        value.unwrap_or_default()
    }

    ///Validates if all the required DE were provided
    pub fn is_valid(&self) -> bool {
        //without mti we cant perform validation
        if !self.has_valid_mti() {
            return false;
        }

        //Get required de info
        let mti = self.get_mti().unwrap();

        let mut required_de = HashMap::new();
        required_de.insert(iso8583::constants::AUTHORIZATION_REQUEST, REQUIRED_DE_0100);
        required_de.insert(iso8583::constants::REVERSAL_REQUEST, REQUIRED_DE_0400);

        let required = required_de[mti.as_str()];

        let vec: Vec<&str> = required.split('|').collect();

        vec.iter()
            .all(|&de| self.fields.iter().any(|field| field.id.as_str() == de))
    }

    //Validates if MTI was informed
    pub fn has_valid_mti(&self) -> bool {
        self.get_mti().is_some()
    }

    pub fn get_mti(&self) -> Option<String> {
        self.get_info(iso8583::constants::MESSAGE_TYPE_INDICATOR)
    }
}

impl TryFrom<&ISORequest> for iso8583::ISOMessage {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(iso8583::ISOMessageError::RequiredDE);
        }

        let mti = iso8583::MessageTypeIndicator::try_from(request)?;

        Ok(Self {
            mti,
            pcode: iso8583::PCode::try_from(request)?,
            pem: iso8583::POSEntryMode::try_from(request)?,
            card: iso8583::Card::try_from(request)?,
            password: iso8583::Password::try_from(request)?,
        })
    }
}

impl TryFrom<&ISORequest> for iso8583::Card {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(iso8583::ISOMessageError::RequiredDE);
        }

        Ok(Self {
            sequence: request.get_evaluated_info(iso8583::constants::CARD_SEQUENCE),
            number: request.get_evaluated_info(iso8583::constants::CARD_NUMBER),
            expiration_date: request.get_evaluated_info(iso8583::constants::CARD_EXPIRATION_DATE),
        })
    }
}

impl TryFrom<&ISORequest> for iso8583::Password {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(iso8583::ISOMessageError::RequiredDE);
        }

        Ok(Self {
            value: request.get_evaluated_info(iso8583::constants::CARD_PASSWORD),
        })
    }
}

impl TryFrom<&ISORequest> for iso8583::MessageTypeIndicator {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        let mti = match request.get_mti() {
            Some(v) => v,
            None => return Err(iso8583::ISOMessageError::UnsuppotedMTI),
        };

        match mti.as_str() {
            "0100" => Ok(iso8583::MessageTypeIndicator::AuthorizationRequest),
            "0400" => Ok(iso8583::MessageTypeIndicator::ReversalRequest),
            _ => Err(iso8583::ISOMessageError::UnsuppotedMTI),
        }
    }
}

impl TryFrom<&ISORequest> for iso8583::PCode {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(iso8583::ISOMessageError::RequiredDE);
        }

        let pcode = request.get_evaluated_info(iso8583::constants::PCODE);
        let pcode = &pcode[0..2];

        match pcode {
            "00" => Ok(iso8583::PCode::Purchase),
            "01" => Ok(iso8583::PCode::Withdraw),
            "17" => Ok(iso8583::PCode::WithdrawDisbursement),
            "20" => Ok(iso8583::PCode::Consultation),
            "28" => Ok(iso8583::PCode::Charge),
            _ => Err(iso8583::ISOMessageError::UnsupportedPCode),
        }
    }
}

impl TryFrom<&ISORequest> for iso8583::POSEntryMode {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(iso8583::ISOMessageError::RequiredDE);
        }

        let pem = request.get_evaluated_info(iso8583::constants::PEM);
        let pem = &pem[0..2];

        match pem {
            "01" => Ok(iso8583::POSEntryMode::Manual),
            "02" => Ok(iso8583::POSEntryMode::MagneticStripe),
            "05" => Ok(iso8583::POSEntryMode::Chip),
            "07" => Ok(iso8583::POSEntryMode::Contactless),
            "10" => Ok(iso8583::POSEntryMode::CredentialOnFile),
            "79" => Ok(iso8583::POSEntryMode::HybridTerminal),
            "80" => Ok(iso8583::POSEntryMode::MagneticStripeRead),
            "81" => Ok(iso8583::POSEntryMode::EletronicCommerce),
            "90" => Ok(iso8583::POSEntryMode::AutoEntryMagneticStripe),
            _ => Err(iso8583::ISOMessageError::UnsupportedPEM),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::error;

    #[test]
    fn test_request_should_has_valid_state() {
        let fields = vec![
            Field {
                id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: iso8583::constants::PCODE.to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: iso8583::constants::PEM.to_string(),
                value: "051".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        assert!(request.is_valid());
    }

    #[test]
    fn test_request_should_has_invalid_state() {
        let fields = vec![Field {
            id: iso8583::constants::CARD_NUMBER.to_string(),
            value: "5276600404324025".to_string(),
        }];

        let request = ISORequest::new(fields);

        assert!(!request.is_valid());
    }

    #[test]
    fn test_request_should_has_valid_mti() {
        let fields = vec![
            Field {
                id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        assert!(request.has_valid_mti());
        assert_eq!(
            request.get_info(iso8583::constants::MESSAGE_TYPE_INDICATOR),
            Some("0100".to_string())
        );
    }

    #[test]
    fn test_request_should_has_invalid_mti() {
        let fields = vec![
            Field {
                id: "1".to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        assert!(!request.has_valid_mti());
        assert_eq!(
            request.get_info(iso8583::constants::MESSAGE_TYPE_INDICATOR),
            None
        );
    }

    #[test]
    fn test_parse_request_should_be_success() -> Result<(), Box<dyn error::Error>> {
        let fields = vec![
            Field {
                id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: iso8583::constants::PCODE.to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: iso8583::constants::PEM.to_string(),
                value: "051".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        let iso = iso8583::ISOMessage::try_from(&request)?;

        assert_eq!(
            iso.mti,
            iso8583::MessageTypeIndicator::AuthorizationRequest
        );
        assert_eq!(iso.card.number, "5276600404324025");
        assert_eq!(iso.pcode, iso8583::PCode::Purchase);
        assert_eq!(iso.pem, iso8583::POSEntryMode::Chip);

        Ok(())
    }

    #[test]
    fn test_parse_mti_0100_from_request_should_be_success() -> Result<(), Box<dyn error::Error>> {
        let fields = vec![
            Field {
                id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: iso8583::constants::PCODE.to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: iso8583::constants::PEM.to_string(),
                value: "81".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        assert!(request.is_valid());

        let iso = iso8583::ISOMessage::try_from(&request)?;

        assert_eq!(
            iso.mti,
            iso8583::MessageTypeIndicator::AuthorizationRequest
        );

        Ok(())
    }

    #[test]
    fn test_parse_mti_0400_from_request_should_be_success() -> Result<(), Box<dyn error::Error>> {
        let fields = vec![
            Field {
                id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: "0400".to_string(),
            },
            Field {
                id: iso8583::constants::PCODE.to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: iso8583::constants::PEM.to_string(),
                value: "81".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        assert!(request.is_valid());

        let iso = iso8583::ISOMessage::try_from(&request)?;

        assert_eq!(iso.mti, iso8583::MessageTypeIndicator::ReversalRequest);

        Ok(())
    }

    #[test]
    fn test_required_de_0100_error_should_be_required_de() {
        let fields = vec![Field {
            id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
            value: "0100".to_string(),
        }];

        let request = ISORequest::new(fields);

        let iso = iso8583::ISOMessage::try_from(&request);

        assert!(iso.is_err());
        assert_eq!(iso.unwrap_err(), iso8583::ISOMessageError::RequiredDE);
    }

    #[test]
    fn test_required_de_0400_error_should_be_required_de() {
        let fields = vec![Field {
            id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
            value: "0400".to_string(),
        }];

        let request = ISORequest::new(fields);

        let iso = iso8583::ISOMessage::try_from(&request);

        assert!(iso.is_err());
        assert_eq!(iso.unwrap_err(), iso8583::ISOMessageError::RequiredDE);
    }
}
