FROM rust:1.40 as builder
WORKDIR /usr/src/authorizer-mastercard
COPY . .
RUN cargo install --path authorizer-mastercard

FROM debian:buster-slim
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/authorizer-mastercard /usr/local/bin/authorizer-mastercard
CMD ["authorizer-mastercard"]