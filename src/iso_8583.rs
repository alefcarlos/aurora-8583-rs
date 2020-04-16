use std::fmt;

#[derive(PartialEq, Debug)]
pub enum MessageTypeIndicator {
    AuthorizationRequest,
    ReversalRequest,
}

impl fmt::Display for MessageTypeIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageTypeIndicator::AuthorizationRequest => write!(f, "AuthorizationRequest(0100)"),
            MessageTypeIndicator::ReversalRequest => write!(f, "ReversalRequest(0400"),
        }
    }
}

///An struct to represent ISO 8583-1
pub struct ISOMessage {
    pub mti: MessageTypeIndicator,
    pub card: Card,
}

pub struct Card {
    pub number: String,
    pub sequence: String,
}

