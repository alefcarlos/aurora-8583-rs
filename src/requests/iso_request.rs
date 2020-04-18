use super::Field;
use crate::domain::*;
use std::{collections::HashMap, convert::TryFrom};

const REQUIRED_DE_0100: &str = "0|2|3|22";
const REQUIRED_DE_0400: &str = "0|2|3|22";
#[derive(Clone)]
pub struct ISORequest {
    pub fields: Vec<Field>,
}

impl ISORequest {
    pub fn new(fields: Vec<Field>) -> Self {
        ISORequest { fields }
    }
}

impl ISORequest {
    ///Gets value from DE
    pub fn get_info(&self, id: &str) -> Option<String> {
        let item = self.fields.iter().find(|&field| field.id == id);

        match item {
            Some(x) => Some(x.value.clone()),
            None => None,
        }
    }

    ///Gets value from DE
    pub fn get_evaluated_info(&self, id: &str) -> String {
        let value = self.get_info(id);
        value.unwrap_or_default()
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
        required_de.insert(AUTHORIZATION_REQUEST, REQUIRED_DE_0100);
        required_de.insert(REVERSAL_REQUEST, REQUIRED_DE_0400);

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
        self.get_info(MESSAGE_TYPE_INDICATOR)
    }
}

impl TryFrom<&ISORequest> for ISOMessage {
    type Error = ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(ISOMessageError::RequiredDE);
        }

        let mti = MessageTypeIndicator::try_from(request)?;

        Ok(Self {
            mti,
            pcode: PCode::try_from(request)?,
            pem: POSEntryMode::try_from(request)?,
            card: Card::try_from(request)?,
            password: Password::try_from(request)?,
        })
    }
}

impl TryFrom<&ISORequest> for Card {
    type Error = ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(ISOMessageError::RequiredDE);
        }

        Ok(Self {
            sequence: request.get_evaluated_info(CARD_SEQUENCE),
            number: request.get_evaluated_info(CARD_NUMBER),
        })
    }
}

impl TryFrom<&ISORequest> for Password {
    type Error = ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(ISOMessageError::RequiredDE);
        }

        Ok(Self {
            value: request.get_evaluated_info(CARD_PASSWORD),
        })
    }
}

impl TryFrom<&ISORequest> for MessageTypeIndicator {
    type Error = ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        let mti = match request.get_mti() {
            Some(v) => v,
            None => return Err(ISOMessageError::UnsuppotedMTI),
        };

        match mti.as_str() {
            "0100" => Ok(MessageTypeIndicator::AuthorizationRequest),
            "0400" => Ok(MessageTypeIndicator::ReversalRequest),
            _ => Err(ISOMessageError::UnsuppotedMTI),
        }
    }
}

impl TryFrom<&ISORequest> for PCode {
    type Error = ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(ISOMessageError::RequiredDE);
        }

        let pcode = request.get_evaluated_info(PCODE);
        let pcode = &pcode[0..2];

        match pcode {
            "00" => Ok(PCode::Purchase),
            "01" => Ok(PCode::Withdraw),
            "20" => Ok(PCode::Consultation),
            "17" => Ok(PCode::WithdrawDisbursement),
            "28" => Ok(PCode::Charge),
            _ => Err(ISOMessageError::UnsupportedPCode),
        }
    }
}

impl TryFrom<&ISORequest> for POSEntryMode {
    type Error = ISOMessageError;

    fn try_from(request: &ISORequest) -> Result<Self, Self::Error> {
        if !request.is_valid() {
            return Err(ISOMessageError::RequiredDE);
        }

        let pem = request.get_evaluated_info(PEM);
        let pem = &pem[0..2];

        match pem {
            "01" => Ok(POSEntryMode::Manual),
            "02" => Ok(POSEntryMode::MagneticStripe),
            "05" => Ok(POSEntryMode::Chip),
            "07" => Ok(POSEntryMode::Contactless),
            "10" => Ok(POSEntryMode::CredentialOnFile),
            "79" => Ok(POSEntryMode::HybridTerminal),
            "80" => Ok(POSEntryMode::MagneticStripeRead),
            "81" => Ok(POSEntryMode::EletronicCommerce),
            "90" => Ok(POSEntryMode::AutoEntryMagneticStripe),
            _ => Err(ISOMessageError::UnsupportedPEM),
        }
    }
}
