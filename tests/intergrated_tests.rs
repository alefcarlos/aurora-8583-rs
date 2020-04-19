use authflow::authorization_iso_8583::iso_8583;
use authflow::requests;
use authflow::domain;

mod tests {
    use super::*;
    use std::convert::TryFrom;
    
    #[test]
    fn test_flow() {
        let fields = vec![
            requests::Field {
                id: iso_8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: iso_8583::constants::AUTHORIZATION_REQUEST.to_string(),
            },
            requests::Field {
                id: iso_8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            requests::Field {
                id: iso_8583::constants::PCODE.to_string(),
                value: "000000".to_string(),
            },
            requests::Field {
                id: iso_8583::constants::CARD_EXPIRATION_DATE.to_string(),
                value: "2416".to_string(),
            },
            requests::Field {
                id: iso_8583::constants::PEM.to_string(),
                value: "81".to_string(),
            },
        ];

        //Incoming request
        let request = requests::ISORequest::new(fields);

        //ApiHandle

        //Aplicar formatador de entrada
        let transaction = domain::TransactionType::try_from(&request);
        assert!(transaction.is_ok(), true);

        let transaction = match transaction {
            Ok(v) => v,
            _ => domain::TransactionType::None,
        };

        //Se TransactionType::None retornar 400 - Bad Request
        assert!(transaction != domain::TransactionType::None, true);

        //Executar flow
        let authorizer_result = domain::authorizer::execute(&transaction);

        assert!(authorizer_result.is_err(), true);

        let result_param = requests::ISOResponsePrepareParams {
            request,
            transaction,
            authorizer_result,
        };

        //Aplicar formatador de saída
        let iso_response = requests::ISOResponse::from(result_param);
        let de_30 = iso_response.get_info("30");

        assert!(de_30.is_some(), true);

        let de_1 = iso_response.get_info("1");
        assert!(de_1.is_none(), true);
    }
}
