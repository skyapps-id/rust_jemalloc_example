FROM rust:1.85 AS builder

WORKDIR /usr/src/actix_app_test

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/actix_app_test/target/release/actix_app_test /usr/local/bin/actix_app_test

RUN chmod +x /usr/local/bin/actix_app_test

ENV RUST_LOG=info

EXPOSE 8080

CMD ["actix_app_test"]
