use crate::domain::TransactionType;

pub fn execute(trx: &TransactionType) -> Result<super::Result, super::Error> {
    //TODO: persistir transacao no banco

    //TODO: emitir evento de transacao recebida

    let result = match trx {
        TransactionType::OlinePurchase(message) => super::online_purchase::execute(&message),
        _ => Err(super::Error::InvalidTransaction)
    };

    let response  = match result {
        Ok(v) => Ok(v),
        Err(v) => Err(v)
    };

    //TODO: emitir evento de transacao negada/aprovada

    response
}