mod models;

pub use models::*;

use crate::domain::{
    validations::card::{validate_cvc, validate_expiration},
    Transactions,
};
use aurora_8583::{iso8583::ISOMessage, Authorizer as AuroraAuth};

pub struct Authorizer<'a> {
    authorizer: AuroraAuth<'a, Transactions>,
}

type ExecutorResult = Result<MyResult, MyError>;

impl<'a> Authorizer<'a> {
    pub fn new() -> Self {
        let mut authorizer = AuroraAuth::<Transactions>::new();

        authorizer.add_validation(Transactions::OnlinePurchase, &validate_cvc);
        authorizer.add_validation(Transactions::OnlinePurchase, &validate_expiration);

        Self { authorizer }
    }

    pub fn execute(&self, trx: &Transactions, iso_message: &ISOMessage) -> ExecutorResult {
        //TODO: persistir transacao no banco

        //TODO: emitir evento de transacao recebida

        let result = self.authorizer.perform(trx, iso_message)?;

        //TODO: emitir evento de transacao negada/aprovada

        Ok(result.into())
    }
}
