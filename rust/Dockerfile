# Build the Rust WebSocket game server
FROM rust:1.82-slim AS builder

WORKDIR /app
COPY rust/ ./rust/
COPY client/ ./client/

WORKDIR /app/rust
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/rust/target/release/rust /app/voidrunner
COPY --from=builder /app/client /app/client

EXPOSE 10000

CMD ["/app/voidrunner"]
