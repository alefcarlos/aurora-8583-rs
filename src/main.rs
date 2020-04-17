mod domain;
mod requests;

use domain::ISOMessage;
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
            value: "051".to_string(),
        },
    ];

    let request = ISORequest::new(fields);
    let iso = ISOMessage::try_from(&request);

    match iso {
        Ok(result) => println!("{}", result.mti),
        _ => print!("acabou"),
    }
}
