use aurora_8583::iso8583;
use std::hash::Hash;

use std::convert::TryFrom;

#[derive(PartialEq, Debug, Hash)]
pub enum Transactions {
    OnlinePurchase,
    None,
}

impl Eq for Transactions {}

impl TryFrom<&iso8583::ISOMessage> for Transactions {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: &iso8583::ISOMessage) -> Result<Self, Self::Error> {
        match request {
            iso8583::ISOMessage {
                mti: iso8583::MessageTypeIndicator::AuthorizationRequest,
                pem: iso8583::POSEntryMode::EletronicCommerce,
                pcode: iso8583::PCode::Purchase,
                ..
            } => Ok(Transactions::OnlinePurchase),
            _ => Err(iso8583::ISOMessageError::UnsupportedTransaction),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::requests::*;

    #[test]
    fn test_transaction_kind_should_be_invalid() {
        let fields = vec![
            Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: "3".to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: "22".to_string(),
                value: "90".to_string(),
            },
        ];

        let request = ISORequest::new(fields);
        assert!(request.is_valid());

        let iso = iso8583::ISOMessage::try_from(&request);
        assert!(iso.is_ok());

        let iso = iso.unwrap();

        let transaction_kind = Transactions::try_from(&iso);

        assert!(transaction_kind.is_err());

        assert_eq!(
            transaction_kind.unwrap_err(),
            iso8583::ISOMessageError::UnsupportedTransaction
        );
    }

    #[test]
    fn test_transaction_kind_should_be_online_purchase() {
        let fields = vec![
            Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: "3".to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: "22".to_string(),
                value: "81".to_string(),
            },
        ];

        let request = ISORequest::new(fields);
        assert!(request.is_valid());

        let iso = iso8583::ISOMessage::try_from(&request);
        assert!(iso.is_ok());

        let iso = iso.unwrap();

        let transaction_kind = Transactions::try_from(&iso);

        assert!(transaction_kind.is_ok());
    }
}
