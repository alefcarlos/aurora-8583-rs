use crate::iso8583::ISOMessage;
use std::{collections::HashMap, hash::Hash};

// TODO: decide the corect types to use here
pub struct Unauthorized(pub String);
pub struct Authorized(pub String, pub u32);

pub struct Unvalidated(pub String);
pub struct Validated;

pub type AuthResult = Result<Authorized, Unauthorized>;
pub type ValidateResult = Result<Validated, Unvalidated>;

type ValidatorCallback = dyn Fn(&ISOMessage) -> ValidateResult;

pub struct Authorizer<'a, T>
where
    T: Eq + Hash,
{
    validations: HashMap<T, Vec<&'a ValidatorCallback>>,
}

impl<'a, T: Eq + Hash> Default for Authorizer<'a, T> {
    fn default() -> Self {
        Self {
            validations: HashMap::new(),
        }
    }
}

impl<'a, T: Eq + Hash> Authorizer<'a, T> {
    pub fn add_validation(&mut self, transaction: T, fun: &'a ValidatorCallback) {
        let callbacks = self
            .validations
            .entry(transaction)
            .or_insert_with(Vec::new);

        callbacks.push(fun);
    }

    // pode receber a request direto também
    pub fn perform(&self, transaction: &T, iso_message: &ISOMessage) -> AuthResult {
        let callbacks = match self.validations.get(transaction) {
            Some(cb) => cb,
            None => {
                return Err(Unauthorized(
                    "Validation not found for this transaction type".to_owned(),
                ))
            }
        };

        for fun in callbacks.iter() {
            fun(iso_message)?;
        }

        Ok(Authorized("Successfully authorized.".to_owned(), 201))
    }
}

impl From<Unvalidated> for Unauthorized {
    fn from(validation: Unvalidated) -> Self {
        Unauthorized(validation.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iso8583::{Card, MessageTypeIndicator, PCode, POSEntryMode, Password};

    #[derive(Hash, PartialEq)]
    enum MyTransaction {
        Online,
        Gift,
    }

    impl Eq for MyTransaction {}

    #[test]
    fn should_valid() {
        let mut authorizer = Authorizer::<MyTransaction>::default();

        authorizer.add_validation(MyTransaction::Online, &|_iso| Ok(Validated));
        authorizer.add_validation(MyTransaction::Online, &|_iso| Ok(Validated));

        let iso = generate_iso_message();
        let result = authorizer.perform(&MyTransaction::Online, &iso);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn should_fail() {
        let mut authorizer = Authorizer::<MyTransaction>::default();

        authorizer.add_validation(MyTransaction::Gift, &|_iso| Ok(Validated));
        authorizer.add_validation(MyTransaction::Gift, &|_iso| {
            Err(Unvalidated("deu ruim".to_owned()))
        });
        let iso = generate_iso_message();

        let result = authorizer.perform(&MyTransaction::Gift, &iso);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_fail_when_validation_is_not_found() {
        let mut authorizer = Authorizer::<MyTransaction>::default();

        authorizer.add_validation(MyTransaction::Online, &|_iso| {
            Err(Unvalidated("deu ruim".to_owned()))
        });

        let iso = generate_iso_message();

        let result = authorizer.perform(&MyTransaction::Gift, &iso);

        assert_eq!(result.is_err(), true);
        // assert_eq!(result.err(), Err(Unauthorized(
        //     "Validation not found for this transaction type".to_owned(),
        // )));
    }

    fn generate_iso_message() -> ISOMessage {
        ISOMessage {
            mti: MessageTypeIndicator::AuthorizationRequest,
            pcode: PCode::Purchase,
            pem: POSEntryMode::EletronicCommerce,
            card: Card {
                number: "12312312".to_owned(),
                sequence: "231312".to_owned(),
                expiration_date: "2023-04-05".to_owned(),
            },
            password: Password {
                value: "fdsafasdfdas".to_owned(),
            },
        }
    }
}
