FROM rust:1.54-slim
WORKDIR /app
RUN apt update && apt install -y pkg-config libssl-dev
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
RUN cargo build --release
ENTRYPOINT [ "/app/target/release/sqlx_transaction_panic" ]
