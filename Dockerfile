FROM rust:1.40 as builder
WORKDIR /usr/src/authflow
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/authflow /usr/local/bin/authflow
CMD ["authflow"]