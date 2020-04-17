use std::{error::Error, fmt};

#[derive(PartialEq, Debug)]
pub enum ISOMessageError {
    RequiredDE,
    UnsuppotedMTI,
    UnsupportedPCode,
    UnsupportedPEM,
}

impl Error for ISOMessageError {}

impl fmt::Display for ISOMessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ISOMessageError::RequiredDE => write!(f, "Required DE were not provided."),
            ISOMessageError::UnsuppotedMTI => write!(f, "Value is not valid for MessageTypeIndicator."),
            ISOMessageError::UnsupportedPCode => write!(f, "Value is not valid for PCode."),
            ISOMessageError::UnsupportedPEM => write!(f, "Value is not valid for PEM."),
        }
    }
}
