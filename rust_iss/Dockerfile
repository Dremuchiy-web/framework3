FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust_iss /app/rust_iss
COPY --from=builder /app/migrations /app/migrations

EXPOSE 3000

CMD ["/app/rust_iss"]

