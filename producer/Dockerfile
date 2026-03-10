# producer\Dockerfile
FROM rust:1.88 AS builder
WORKDIR /app

COPY . .
RUN cargo build --release -p producer

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/producer /usr/local/bin/producer

CMD ["producer"]