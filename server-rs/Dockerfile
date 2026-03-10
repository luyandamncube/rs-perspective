# server-rs\Dockerfile
FROM rust:1.88 AS builder
WORKDIR /app

COPY . .
RUN cargo build --release -p server-rs

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server-rs /usr/local/bin/server-rs
COPY --from=builder /app/server-rs/static /app/static

EXPOSE 3000

ENV STATIC_DIR=/app/static

CMD ["server-rs"]