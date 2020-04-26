use aurora_8583::iso8583;
use authorizer_mastercard::domain;
use authorizer_mastercard::requests;
use std::convert::TryFrom;
use std::error;

mod tests {
    use super::*;

    #[test]
    fn test_should_ok_when_request_is_valid() -> Result<(), Box<dyn error::Error>> {
        let fields = vec![
            requests::Field {
                id: iso8583::constants::MESSAGE_TYPE_INDICATOR.to_string(),
                value: iso8583::constants::AUTHORIZATION_REQUEST.to_string(),
            },
            requests::Field {
                id: iso8583::constants::CARD_NUMBER.to_string(),
                value: "5276600404324025".to_string(),
            },
            requests::Field {
                id: iso8583::constants::PCODE.to_string(),
                value: "000000".to_string(),
            },
            requests::Field {
                id: iso8583::constants::CARD_EXPIRATION_DATE.to_string(),
                value: "2416".to_string(),
            },
            requests::Field {
                id: iso8583::constants::PEM.to_string(),
                value: "81".to_string(),
            },
        ];

        //Incoming request
        let request = requests::ISORequest::new(fields);

        //ApiHandle

        // Converte request em Message
        let iso = iso8583::ISOMessage::try_from(&request)?;

        //Aplicar formatador de entrada
        let transaction = domain::Transactions::try_from(&iso)?;

        //Se TransactionType::None retornar 400 - Bad Request
        assert!(
            transaction != domain::Transactions::None,
            "Transaction type must be different from TransactionType::None"
        );

        let authorizer = domain::authorizer::Authorizer::new();

        //Executar flow
        let authorizer_result = authorizer.execute(&transaction, &iso);

        let result_param = requests::ISOResponsePrepareParams {
            request,
            transaction,
            authorizer_result,
        };

        //Aplicar formatador de saída
        let iso_response = requests::ISOResponse::from(result_param);
        let de_0 = iso_response
            .get_info(iso8583::constants::MESSAGE_TYPE_INDICATOR)
            .unwrap();

        assert_eq!(de_0, "0110", "de 0 should be 0110, but is {}", de_0);

        let de_30 = iso_response.get_info("30").unwrap();

        assert_eq!(de_30, "00", "de 30 should be 00");

        // let de_1 = iso_response.get_info("1");
        // assert!(de_1.is_none(), );

        Ok(())
    }
}
