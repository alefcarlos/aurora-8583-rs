use std::{collections::HashMap, convert::TryFrom};
use super::Field;
use crate::domain::*;

const REQUIRED_DE_0100: &str = "0|2";
const REQUIRED_DE_0400: &str = "0|1|2";

pub struct ISORequest {
    fields: Vec<Field>,
}

impl ISORequest {
    pub fn new(fields: Vec<Field>) -> Self{
        ISORequest {
            fields
        }
    }
}

impl ISORequest {
    ///Gets value from DE
    pub fn get_info(&self, id: String) -> Option<String> {
        let item = self.fields.iter().find(|&field| field.id == id);

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

        vec.iter()
            .all(|&de| self.fields.iter().any(|field| field.id.as_str() == de))
    }

    //Validates if MTI was informed
    pub fn has_valid_mti(&self) -> bool {
        self.get_mti().is_some()
    }

    pub fn get_mti(&self) -> Option<String> {
        self.get_info("0".to_string())
    }
}

impl TryFrom<&ISORequest> for ISOMessage {
    type Error = &'static str;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err("Request has an invalid state!");
        }

        let mti = MessageTypeIndicator::try_from(request)?;

        return Ok(ISOMessage {
            mti,
            card: Card::try_from(request)?,
            password: Password::try_from(request)?
        });
    }
}

impl TryFrom<&ISORequest> for Card {
    type Error = &'static str;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err("Request has an invalid state!");
        }

        Ok(Card {
            sequence: "0".to_string(),
            number: request.get_evaluated_info("2".to_string()),
        })
    }
}

impl TryFrom<&ISORequest> for Password {
    type Error = &'static str;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err("Request has an invalid state!");
        }

        Ok(Password {
            value: request.get_evaluated_info("52".to_string()),
        })
    }
}

impl TryFrom<&ISORequest> for MessageTypeIndicator {
    type Error = &'static str;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        let mti = request.get_mti();

        if mti.is_none() {
            return Err("Request has an invalid MTI!");
        }

        //QUE BOXTA: https://stackoverflow.com/questions/48034119/how-can-i-pattern-match-against-an-optionstring
        let mti = mti.as_ref().map(String::as_str);

        return match mti {
            Some("0100") => Ok(MessageTypeIndicator::AuthorizationRequest),
            Some("0400") => Ok(MessageTypeIndicator::ReversalRequest),
            _ => Err("MTI is not supported"),
        };
    }
}
