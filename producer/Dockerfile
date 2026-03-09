# producer\Dockerfile
FROM rust:1.88 AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    pkg-config \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./
RUN mkdir -p src && \
    printf 'fn main() {}\n' > src/main.rs && \
    printf '' > src/lib.rs && \
    cargo fetch

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/producer /usr/local/bin/producer

CMD ["producer"]