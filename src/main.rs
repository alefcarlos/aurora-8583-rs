use std::fmt;

const REQUIRED_DE_0100: &str = "0|2";
const REQUIRED_DE_0400: &str = "0|1|2";

fn main() {
    println!("Hello, world!");

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
    println!("Total: {}", request.fields.capacity());

    println!("requrest is valid? {}", request.validate());

    let mti = request.get_info("0".to_string());

    match mti {
        None => println!("MTI was not provided"),
        Some(x) => println!("mti: {}", x),
    }

    let iso = ISOMessage::from(request);
    println!("enum mti {}", iso.mti);
}

struct ISORequestMessage {
    fields: Vec<Field>,
}

impl ISORequestMessage {
    ///Gets value from DE
    fn get_info(&self, id: String) -> Option<String> {
        let item = self.fields.iter().find(|field: &&Field| field.id == id);

        return match item {
            None => None,
            Some(x) => Some(x.value.clone()),
        };
    }

    ///Validates if all the required DE were provided
    fn validate(&self) -> bool {
        //without mti we cant perform validation
        if !self.validate_mti() {
            return false;
        }

        //Get required de info

        return true;
    }

    //Validates if MTI was informed
    fn validate_mti(&self) -> bool {
        return match self.get_info("0".to_string()) {
            Some(_) => true,
            _ => false,
        };
    }
}

struct Field {
    id: String,
    value: String,
}
#[derive(PartialEq)]
#[derive(Debug)]
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

///An struct to represent ISO 8583-1
struct ISOMessage {
    mti: MessageTypeIndicator,
    card_number: String,
}

impl ISOMessage {
    fn from(request: ISORequestMessage) -> ISOMessage {
        return ISOMessage {
            mti: MessageTypeIndicator::AuthorizationRequest,
            card_number: "5276600404324025".to_string(),
        };
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

        assert_eq!(request.validate(), true);
    }

    #[test]
    fn test_request_should_has_invalid_state() {
        let fields = vec![Field {
            id: "2".to_string(),
            value: "5276600404324025".to_string(),
        }];

        let request = ISORequestMessage { fields };

        assert_eq!(request.validate(), false);
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

        assert_eq!(request.validate_mti(), true);
        assert_eq!(request.get_info("0".to_string()), Some("0100".to_string()));
    }

    #[test]
    fn test_request_should_has_invalid_mti() {
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

        assert_eq!(request.validate_mti(), false);
        assert_eq!(request.get_info("0".to_string()), None);
    }

    #[test]
    fn test_parse_request_should_be_success() {
        let fields = vec![Field {
            id: "2".to_string(),
            value: "5276600404324025".to_string(),
        }];

        let request = ISORequestMessage { fields };

        let iso = ISOMessage::from(request);

        assert_eq!(iso.mti, MessageTypeIndicator::AuthorizationRequest);
    }
}
