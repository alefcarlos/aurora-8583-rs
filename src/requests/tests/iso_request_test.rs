mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::domain::*;
    use crate::requests::*;
    use std::convert::TryFrom;

    #[test]
    fn test_request_should_has_valid_state() {
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

        let request = ISORequest::new(fields);

        assert_eq!(request.is_valid(), true);
    }

    #[test]
    fn test_request_should_has_invalid_state() {
        let fields = vec![Field {
            id: "2".to_string(),
            value: "5276600404324025".to_string(),
        }];

        let request = ISORequest::new(fields);

        assert_eq!(request.is_valid(), false);
    }

    #[test]
    fn test_request_should_has_valid_mti() {
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

        let request = ISORequest::new(fields);

        assert_eq!(request.has_valid_mti(), true);
        assert_eq!(request.get_info("0".to_string()), Some("0100".to_string()));
    }

    #[test]
    fn test_request_should_has_invalid_mti() {
        let fields = vec![
            Field {
                id: "1".to_string(),
                value: "0100".to_string(),
            },
            Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
        ];

        let request = ISORequest::new(fields);

        assert_eq!(request.has_valid_mti(), false);
        assert_eq!(request.get_info("0".to_string()), None);
    }

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

        let request = ISORequest::new(fields);

        let iso = ISOMessage::try_from(&request);

        assert_eq!(iso.is_ok(), true);

        let unwrap_iso = iso.unwrap();
        assert_eq!(unwrap_iso.mti, MessageTypeIndicator::AuthorizationRequest);
        assert_eq!(unwrap_iso.card.number, "5276600404324025");
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

        let request = ISORequest::new(fields);

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

        let request = ISORequest::new(fields);

        let iso = ISOMessage::try_from(&request);

        assert_eq!(iso.is_err(), true);
    }
}
