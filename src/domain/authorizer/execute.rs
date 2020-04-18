// use crate::domain::TransactionType;
// use super::online_purchase::execute_online_purchase;


// // pub fn execute_auth_flow(trx: &TransactionType) -> Result<self::Result, Error> {
// //     //TODO: persistir transacao no banco

// //     //TODO: emitir evento de transacao recebida

// //     let result = match trx {
// //         TransactionType::OlinePurchase(message) => execute_online_purchase(&message),
// //         _ => Err(Error::InvalidTransaction)
// //     };

// //     let response  = match result {
// //         Ok(v) => Ok(v),
// //         Err(v) => Err(v)
// //     };

// //     //TODO: emitir evento de transacao negada/aprovada

// //     response
// // }