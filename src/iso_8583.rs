use std::{convert::TryFrom, fmt};
use crate::request::ISORequestMessage;

#[derive(PartialEq, Debug)]
pub enum MessageTypeIndicator {
    AuthorizationRequest,
    ReversalRequest,
}

impl TryFrom<&ISORequestMessage> for MessageTypeIndicator {
    type Error = &'static str;

    fn try_from(request: &ISORequestMessage) -> Result<Self, Self::Error> {
        let mti = request.get_mti();

        if mti.is_none() {
            return Err("Request has an invalid MTI!");
        }

        //QUE BOXTA: https://stackoverflow.com/questions/48034119/how-can-i-pattern-match-against-an-optionstring
        let mti = mti.as_ref().map(String::as_str);

        return match mti {
            Some("0100") => Ok(MessageTypeIndicator::AuthorizationRequest),
            Some("0400") => Ok(MessageTypeIndicator::ReversalRequest),
            _ => Err("MTI is not supported"),
        };
    }
}

///An struct to represent ISO 8583-1
pub struct ISOMessage {
    mti: MessageTypeIndicator,
    card_number: String,
}

impl fmt::Display for MessageTypeIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageTypeIndicator::AuthorizationRequest => write!(f, "AuthorizationRequest(0100)"),
            MessageTypeIndicator::ReversalRequest => write!(f, "ReversalRequest(0400"),
        }
    }
}

impl TryFrom<&ISORequestMessage> for ISOMessage {
    type Error = &'static str;

    fn try_from(request: &ISORequestMessage) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err("Request has an invalid state!");
        }

        let mti = MessageTypeIndicator::try_from(request)?;

        return Ok(ISOMessage {
            mti,
            card_number: request.get_evaluated_info("2".to_string()),
        });
    }
}


mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::request::Field;
    use crate::request::ISORequestMessage;

    #[test]
    fn test_parse_request_should_be_success() {
        let fields = vec![
            Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
        ];

        let request = ISORequestMessage { fields };

        let iso = ISOMessage::try_from(&request);
        
        assert_eq!(iso.is_ok(), true);

        let unwrap_iso = iso.unwrap();
        assert_eq!(unwrap_iso.mti, MessageTypeIndicator::AuthorizationRequest);
        assert_eq!(unwrap_iso.card_number, "5276600404324025");
    }

    #[test]
    fn test_parse_mti_from_request_should_be_success() {
        let fields = vec![
            Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
        ];

        let request = ISORequestMessage { fields };
        assert_eq!(request.is_valid(), true);

        let iso = ISOMessage::try_from(&request);
        assert_eq!(iso.is_ok(), true);

        let unwrap_iso = iso.unwrap();
        assert_eq!(unwrap_iso.mti, MessageTypeIndicator::AuthorizationRequest);
    }

    #[test]
    fn test_parse_mti_from_request_should_be_invalid() {
        let fields = vec![Field {
            id: "1".to_string(),
            value: "0100".to_string(),
        }];

        let request = ISORequestMessage { fields };

        let iso = ISOMessage::try_from(&request);

        assert_eq!(iso.is_err(), true);
    }
}
