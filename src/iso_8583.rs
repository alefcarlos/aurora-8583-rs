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
    pub mti: MessageTypeIndicator,
    pub card_number: String,
}

impl fmt::Display for MessageTypeIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageTypeIndicator::AuthorizationRequest => write!(f, "AuthorizationRequest(0100)"),
            MessageTypeIndicator::ReversalRequest => write!(f, "ReversalRequest(0400"),
        }
    }
}

