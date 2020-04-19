use super::{Field, ISORequest};
use crate::domain;
use aurora_8583::iso8583;

pub struct ISOResponsePrepareParams {
    pub request: ISORequest,
    pub transaction: domain::TransactionType,
    pub authorizer_result: Result<domain::authorizer::Result, domain::authorizer::Error>,
}

pub struct ISOResponse {
    fields: Vec<Field>,
}

impl ISOResponse {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }

    fn add_field(&mut self, value: Field) {
        self.fields.push(value);
    }

    fn add_value_field(&mut self, id: String, value: String) {
        self.add_field(Field { id, value });
    }

    fn rm_field(&mut self, id: &str) {
        let index = self.fields.iter().position(|f| f.id == id);

        match index {
            Some(v) => self.fields.remove(v),
            None => return (),
        };
    }

    ///Gets value from DE
    pub fn get_info(&self, id: &str) -> Option<String> {
        let item = self.fields.iter().find(|&field| field.id == id);

        match item {
            Some(x) => Some(x.value.clone()),
            None => None,
        }
    }
}

impl From<ISOResponsePrepareParams> for ISOResponse {
    fn from(value: ISOResponsePrepareParams) -> Self {
        let mut response = Self::from(value.request);

        // Delete default DE
        response.rm_field(iso8583::constants::MESSAGE_TYPE_INDICATOR);

        //TODO: remover DE de acordo com a transaction
        match value.transaction {
            domain::TransactionType::OlinePurchase(_) => {
                response.add_value_field(iso8583::constants::MESSAGE_TYPE_INDICATOR.to_owned(), "0110".to_owned());
            }
            _ => (),
        }

        //TODO: aplicar novos DE

        response.add_value_field(
            iso8583::constants::RESPONSE_CODE.to_owned(),
            "00".to_owned(),
        );

        response
    }
}

impl From<ISORequest> for ISOResponse {
    fn from(value: ISORequest) -> Self {
        let mut this = Self::new();

        for f in value.fields.iter() {
            this.add_field(Field {
                id: f.id.clone(),
                value: f.value.clone(),
            });
        }

        this
    }
}
