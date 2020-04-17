mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::domain::*;
    use crate::requests::*;
    use std::convert::TryFrom;

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
        assert_eq!(request.is_valid(), true);

        let iso = ISOMessage::try_from(&request);
        assert_eq!(iso.is_ok(), true);

        let transaction_kind = TransactionKind::try_from(&iso.unwrap());

        assert_eq!(transaction_kind.is_err(), true);
        assert_eq!(transaction_kind.unwrap_err(), TransactionErrors::UnsupportedTransaction);
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
        assert_eq!(request.is_valid(), true);

        let iso = ISOMessage::try_from(&request);
        assert_eq!(iso.is_ok(), true);

        let transaction_kind = TransactionKind::try_from(&iso.unwrap());

        assert_eq!(transaction_kind.is_ok(), true);
        assert_eq!(transaction_kind.unwrap(), TransactionKind::OlinePurchase);
    }
}
