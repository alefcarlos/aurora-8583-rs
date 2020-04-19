use crate::{requests, authorization_iso_8583::iso_8583};
use std::convert::TryFrom;

#[derive(PartialEq, Debug)]
pub enum TransactionType {
    OlinePurchase(iso_8583::ISOMessage),
    PresentPurchase(iso_8583::ISOMessage),
    Withdraw(iso_8583::ISOMessage),
    None,
}

impl TryFrom<iso_8583::ISOMessage> for TransactionType {
    type Error = iso_8583::ISOMessageError;

    fn try_from(request: iso_8583::ISOMessage) -> Result<Self, Self::Error> {
        match request {
            iso_8583::ISOMessage {
                mti: iso_8583::MessageTypeIndicator::AuthorizationRequest,
                pem: iso_8583::POSEntryMode::EletronicCommerce,
                pcode: iso_8583::PCode::Purchase,
                ..
            } => Ok(TransactionType::OlinePurchase(request)),
            _ => Err(iso_8583::ISOMessageError::UnsupportedTransaction),
        }
    }
}

impl TryFrom<&requests::ISORequest> for TransactionType {
    type Error = iso_8583::ISOMessageError;

    fn try_from(value: &requests::ISORequest) -> Result<Self, Self::Error> {
        let iso = iso_8583::ISOMessage::try_from(value)?;

        //TODO: Validar DE requeridos de acordo com TransactionType

        TransactionType::try_from(iso)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_transaction_kind_should_be_invalid() {
        let fields = vec![
            requests::Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            requests::Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
            requests::Field {
                id: "3".to_string(),
                value: "000000".to_string(),
            },
            requests::Field {
                id: "22".to_string(),
                value: "90".to_string(),
            },
        ];

        let request = requests::ISORequest::new(fields);
        assert!(request.is_valid());

        let iso = iso_8583::ISOMessage::try_from(&request);
        assert!(iso.is_ok());

        let iso = iso.unwrap();

        let transaction_kind = TransactionType::try_from(iso);

        assert!(transaction_kind.is_err());

        assert_eq!(
            transaction_kind.unwrap_err(),
            iso_8583::ISOMessageError::UnsupportedTransaction
        );
    }

    #[test]
    fn test_transaction_kind_should_be_online_purchase() {
        let fields = vec![
            requests::Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            requests::Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
            requests::Field {
                id: "3".to_string(),
                value: "000000".to_string(),
            },
            requests::Field {
                id: "22".to_string(),
                value: "81".to_string(),
            },
        ];

        let request = requests::ISORequest::new(fields);
        assert!(request.is_valid());

        let iso = iso_8583::ISOMessage::try_from(&request);
        assert!(iso.is_ok());

        let iso = iso.unwrap();

        let transaction_kind = TransactionType::try_from(iso);

        assert!(transaction_kind.is_ok());
    }
}
