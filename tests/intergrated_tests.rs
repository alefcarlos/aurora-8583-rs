use authflow::authorization_iso_8583::iso_8583;
use authflow::requests;
use authflow::domain;

mod tests {
    use super::*;
    use std::{error, convert::TryFrom};
    
    #[test]
    fn test_should_ok_when_request_is_valid() -> Result<(), Box<dyn error::Error>>  {
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
        let transaction = domain::TransactionType::try_from(&request)?;

        //Se TransactionType::None retornar 400 - Bad Request
        assert!(transaction != domain::TransactionType::None, "Transaction type must be different from TransactionType::None");

        //Executar flow
        let authorizer_result = domain::authorizer::execute(&transaction);
        
        assert!(!authorizer_result.is_err(), "authorizer_result must no be Err");

        let result_param = requests::ISOResponsePrepareParams {
            request,
            transaction,
            authorizer_result,
        };

        //Aplicar formatador de saída
        let iso_response = requests::ISOResponse::from(result_param);
        let de_0 = iso_response.get_info(iso_8583::constants::MESSAGE_TYPE_INDICATOR).unwrap();

        assert_eq!(de_0, "0110", "de 0 should be 0110, but is {}", de_0);

        let de_30 = iso_response.get_info("30").unwrap();

        assert_eq!(de_30, "00", "de 30 should be 00");

        // let de_1 = iso_response.get_info("1");
        // assert!(de_1.is_none(), );

        Ok(())
    }
}
