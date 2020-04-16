use std::collections::HashMap;
use std::fmt;

const REQUIRED_DE_0100: &str = "0|2";
const REQUIRED_DE_0400: &str = "0|1|2";

fn main() {
    println!("Hello, world!");

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

    let request = ISORequestMessage { fields };

    if !request.is_valid() {
        panic!("Required DE were not provided");
    }

    let _iso = ISOMessage::from(&request);
    println!("Happy end!");
}

struct ISORequestMessage {
    fields: Vec<Field>,
}

impl ISORequestMessage {
    ///Gets value from DE
    fn get_info(&self, id: String) -> Option<String> {
        let item = self.fields.iter().find(|&field: &&Field| field.id == id);

        return match item {
            None => None,
            Some(x) => Some(x.value.clone()),
        };
    }

    ///Gets value from DE
    fn get_evaluated_info(&self, id: String) -> String {
        let value = self.get_info(id);
        return value.unwrap_or_default();
    }

    ///Validates if all the required DE were provided
    fn is_valid(&self) -> bool {
        //without mti we cant perform validation
        if !self.has_valid_mti() {
            return false;
        }

        //Get required de info
        let mti = self.get_mti().unwrap();

        let mut required_de = HashMap::new();
        required_de.insert("0100", REQUIRED_DE_0100);
        required_de.insert("0400", REQUIRED_DE_0400);

        let required = required_de[mti.as_str()];

        let vec: Vec<&str> = required.split('|').collect();

        return vec
            .iter()
            .all(|&de| self.fields.iter().any(|field| field.id.as_str() == de));
    }

    //Validates if MTI was informed
    fn has_valid_mti(&self) -> bool {
        return match self.get_mti() {
            Some(_) => true,
            _ => false,
        };
    }

    fn get_mti(&self) -> Option<String> {
        return match self.get_info("0".to_string()) {
            Some(x) => Some(x),
            _ => None,
        };
    }
}

struct Field {
    id: String,
    value: String,
}
#[derive(PartialEq, Debug)]
enum MessageTypeIndicator {
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

impl MessageTypeIndicator {
    fn from(request: &ISORequestMessage) -> Option<MessageTypeIndicator> {
        let mti = request.get_mti();

        if mti.is_none() {
            return None;
        }

        //QUE BOXTA: https://stackoverflow.com/questions/48034119/how-can-i-pattern-match-against-an-optionstring
        let mti = mti.as_ref().map(String::as_str);

        return match mti {
            Some("0100") => Some(MessageTypeIndicator::AuthorizationRequest),
            Some("0400") => Some(MessageTypeIndicator::ReversalRequest),
            _ => None,
        };
    }
}

///An struct to represent ISO 8583-1
struct ISOMessage {
    mti: MessageTypeIndicator,
    card_number: String,
}

impl ISOMessage {
    fn from(request: &ISORequestMessage) -> Option<ISOMessage> {
        if !request.is_valid() {
            return None;
        }

        let mti = MessageTypeIndicator::from(&request);
        if mti.is_none() {
            return None;
        }

        return Some(ISOMessage {
            mti: mti.unwrap(),
            card_number: request.get_evaluated_info("2".to_string()),
        });
    }
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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

        let request = ISORequestMessage { fields };

        assert_eq!(request.is_valid(), true);
    }

    #[test]
    fn test_request_should_has_invalid_state() {
        let fields = vec![Field {
            id: "2".to_string(),
            value: "5276600404324025".to_string(),
        }];

        let request = ISORequestMessage { fields };

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

        let request = ISORequestMessage { fields };

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

        let request = ISORequestMessage { fields };

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

        let request = ISORequestMessage { fields };

        let iso = ISOMessage::from(&request);
        assert_eq!(iso.is_some(), true);

        let unwrap_iso = iso.unwrap();
        assert_eq!(unwrap_iso.mti, MessageTypeIndicator::AuthorizationRequest);
        assert_eq!(unwrap_iso.card_number, "5276600404324025");
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

        let request = ISORequestMessage { fields };
        assert_eq!(request.is_valid(), true);

        let iso = ISOMessage::from(&request);
        assert_eq!(iso.is_some(), true);

        let unwrap_iso = iso.unwrap();
        assert_eq!(unwrap_iso.mti, MessageTypeIndicator::AuthorizationRequest);
    }

    #[test]
    fn test_parse_mti_from_request_should_be_invalid() {
        let fields = vec![Field {
            id: "1".to_string(),
            value: "0100".to_string(),
        }];

        let request = ISORequestMessage { fields };

        let iso = ISOMessage::from(&request);

        assert_eq!(iso.is_none(), true);
    }
}
