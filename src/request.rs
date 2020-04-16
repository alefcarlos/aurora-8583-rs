use std::collections::HashMap;

const REQUIRED_DE_0100: &str = "0|2";
const REQUIRED_DE_0400: &str = "0|1|2";

pub struct Field {
    pub id: String,
    pub value: String,
}

pub struct ISORequestMessage {
    pub fields: Vec<Field>,
}

impl ISORequestMessage {
    ///Gets value from DE
    pub fn get_info(&self, id: String) -> Option<String> {
        let item = self.fields.iter().find(|&field: &&Field| field.id == id);

        return match item {
            None => None,
            Some(x) => Some(x.value.clone()),
        };
    }

    ///Gets value from DE
    pub fn get_evaluated_info(&self, id: String) -> String {
        let value = self.get_info(id);
        return value.unwrap_or_default();
    }

    ///Validates if all the required DE were provided
    pub fn is_valid(&self) -> bool {
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
    pub fn has_valid_mti(&self) -> bool {
        return match self.get_mti() {
            Some(_) => true,
            _ => false,
        };
    }

    pub fn get_mti(&self) -> Option<String> {
        return match self.get_info("0".to_string()) {
            Some(x) => Some(x),
            _ => None,
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
}
