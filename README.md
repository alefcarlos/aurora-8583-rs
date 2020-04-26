![Rust](https://github.com/alefcarlos/aurora-8583-rs/workflows/Rust/badge.svg)

[![codecov](https://codecov.io/gh/alefcarlos/aurora-8583-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/alefcarlos/aurora-8583-rs)

# aurora-8583

> aurora-8583 é um conjunto de utilitários para ajudar a escrever fluxo de autorização de uma transação `ISO8583`

# Fluxo de autorização

Com intuito de aprender Rust resolvi reescrever um módulo do processo de autorização transações de cartão de crédito feito por uma Emissora de cartões.

> Será simulado algumas transações `Mastercard` recebida pelo `jPOS`.
> [jPOS](http://www.jpos.org/) handles ISO8583 standard

## ISO8583

Iremos modelar uma [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) e criar  algumas regras de valições dependendo do tipo de transação.
