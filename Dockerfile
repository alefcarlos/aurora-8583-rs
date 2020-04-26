FROM rust:1.40 as builder
WORKDIR /usr/src/authorizer-mastercard
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/authorizer_mastercard /usr/local/bin/authorizer_mastercard
CMD ["authorizer-mastercard"]