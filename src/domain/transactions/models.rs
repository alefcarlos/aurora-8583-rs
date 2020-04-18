use crate::{
    domain::{ISOMessage, ISOMessageError, MessageTypeIndicator, PCode, POSEntryMode},
    requests::ISORequest,
};
use std::convert::TryFrom;

#[derive(PartialEq, Debug)]
pub enum TransactionType {
    OlinePurchase(ISOMessage),
    PresentPurchase(ISOMessage),
    Withdraw(ISOMessage),
    None,
}

impl TryFrom<ISOMessage> for TransactionType {
    type Error = ISOMessageError;

    fn try_from(request: ISOMessage) -> Result<Self, Self::Error> {
        match request {
            ISOMessage {
                mti: MessageTypeIndicator::AuthorizationRequest,
                pem: POSEntryMode::EletronicCommerce,
                pcode: PCode::Purchase,
                ..
            } => Ok(TransactionType::OlinePurchase(request)),
            _ => Err(ISOMessageError::UnsupportedTransaction),
        }
    }
}

impl TryFrom<&ISORequest> for TransactionType {
    type Error = ISOMessageError;

    fn try_from(value: &ISORequest) -> Result<Self, Self::Error> {
        let iso = ISOMessage::try_from(value)?;

        //TODO: Validar DE requeridos de acordo com TransactionType

        TransactionType::try_from(iso)
    }
}
