
use crate::domain::Transactions;
use aurora_8583::TryAuthorizerTransaction;

pub fn execute(trx: &Transactions) -> Result<super::Result, super::Error> {
    //TODO: persistir transacao no banco

    //TODO: emitir evento de transacao recebida

    let result = match trx {
        Transactions::OnlinePurchase(message) => message.try_authorize(),
        _ => Err(super::Error::InvalidTransaction)
    };

    let response  = match result {
        Ok(v) => Ok(v),
        Err(v) => Err(v)
    };

    //TODO: emitir evento de transacao negada/aprovada

    response
}