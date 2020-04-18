mod domain;
mod requests;

use domain::{ISOMessage, TransactionType};
use requests::*;
use std::convert::TryFrom;

fn main() {
    println!("Hello, world!");
}

// fn executar_regra_1(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
//     Ok(ValidationResult::None)
// }

// fn executar_regra_2(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
//     Err(AuthorizerError::SaldoInsuficiente)
// }

// fn executar_regra_3(message: &ISOMessage) -> Result<ValidationResult, AuthorizerError> {
//     Ok(ValidationResult::None)
// }

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::domain::*;
    use crate::requests::*;
    use std::convert::TryFrom;

    #[test]
    fn test_flow() {
        let fields = vec![
            Field {
                id: MESSAGE_TYPE_INDICATOR.to_string(),
                value: AUTHORIZATION_REQUEST.to_string(),
            },
            Field {
                id: CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            Field {
                id: PCODE.to_string(),
                value: "000000".to_string(),
            },
            Field {
                id: PEM.to_string(),
                value: "81".to_string(),
            },
        ];

        //Incoming request
        let request = ISORequest::new(fields);

        //ApiHandle

        //Aplicar formatador de entrada
        let transaction = TransactionType::try_from(&request);
        assert!(transaction.is_ok(), true);

        let transaction = match transaction {
            Ok(v) => v,
            _ => TransactionType::None,
        };

        //Se TransactionType::None retornar 400 - Bad Request
        assert!(transaction != TransactionType::None, true);

        //Executar flow
        let authorizer_result = execute_auth_flow(&transaction);

        let result_param = ISOResponsePrepareParams {
            request,
            transaction,
            authorizer_result,
        };

        //Aplicar formatador de saída
        let iso_response = ISOResponse::from(result_param);
        let de_30 = iso_response.get_info("30");
        
        assert!(de_30.is_some(), true);

        let de_1 = iso_response.get_info("1");
        assert!(de_1.is_none(), true);
    }
}
