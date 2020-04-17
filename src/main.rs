mod domain;
mod requests;

use domain::{ISOMessage, TransactionType};
use requests::*;
use std::convert::TryFrom;

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
        Field {
            id: "3".to_string(),
            value: "000000".to_string(),
        },
        Field {
            id: "22".to_string(),
            value: "81".to_string(),
        },
    ];

    let request = ISORequest::new(fields);

    //Aplicar formatador de entrada
    let transaction = TransactionType::try_from(&request);
    let transaction = transaction.expect("Falha ao tentar fazer o try_from");

    match transaction {
        TransactionType::OlinePurchase(m) => println!("{}", m.mti),
        _ => println!("Acabou"),
    }
}