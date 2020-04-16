FROM rust:1.40 as builder
WORKDIR /usr/src/auth_flow
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/local/cargo/bin/auth_flow /usr/local/bin/auth_flow
CMD ["auth_flow"]