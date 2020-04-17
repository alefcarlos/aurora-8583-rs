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

#[repr(i32)]
#[derive(PartialEq, Debug)]
pub enum PCode {
    Purchase = 0,
    Withdraw = 1,
    Consultation = 20,
    WithdrawDisbursement = 17,
    Charge = 28,
}

#[repr(i32)]
#[derive(PartialEq, Debug)]
pub enum POSEntryMode {
    Manual = 01,
    MagneticStripe = 02,
    Chip = 05,
    Contactless = 07,
    CredentialOnFile = 10,
    HybridTerminal = 79,
    MagneticStripeRead = 80,
    EletronicCommerce = 81,
    AutoEntryMagneticStripe = 90,
}

///An struct to represent ISO 8583-1
#[derive(Debug, PartialEq)]
pub struct ISOMessage {
    /// DE 01
    pub mti: MessageTypeIndicator,

    /// DE 03, Processing code
    pub pcode: PCode,

    /// DE 22, Point of service entry mode
    pub pem: POSEntryMode,

    /// Card information
    pub card: Card,

    /// Password information
    pub password: Password,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    /// DE 02
    pub number: String,

    /// DE 23
    pub sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct Password {
    /// DE 52
    pub value: String,
}
