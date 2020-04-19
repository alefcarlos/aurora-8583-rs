# Fluxo de autorização

    Com intuito de aprender Rust resolvi reescrever um módulo do processo de autorização transações de cartão de crédito feito por uma Emissora de cartões.

> Será simulado algumas transações `Mastercard` recebida pelo `jPOS`.
> [jPOS](http://www.jpos.org/) handles ISO8583 standard

## ISO8583

Iremos modelar uma ISO8583 aplicando algumas regras de valições dependendo do tipo de transação.

## Tipos de transações


## Validações

Para criar uma nova validação é necessário implementar a trait `TryValidate`.


```rust
pub trait TryValidate<T, E> {
    fn try_validate(&self) -> Result<T, E>;
}
```

Validações de exemplo:

- Validar data de expiração
- Validar CVC