use std::convert::TryFrom;
use crate::domain::{MessageTypeIndicator, ISOMessage, POSEntryMode, PCode};
use super::TransactionErrors;

#[derive(PartialEq, Debug)]
pub enum TransactionKind {
    OlinePurchase,
    PresentPurchase,
    Withdraw,
}

impl TryFrom<&ISOMessage> for TransactionKind {
    type Error = TransactionErrors;
    
    fn try_from(request: &ISOMessage) -> Result<Self, Self::Error> { 
        match request {
            ISOMessage {
                mti: MessageTypeIndicator::AuthorizationRequest,
                pem: POSEntryMode::EletronicCommerce,
                pcode: PCode::Purchase,
                ..
            } => Ok(TransactionKind::OlinePurchase),
            _ => Err(TransactionErrors::UnsupportedTransaction),
        }
     }
}
