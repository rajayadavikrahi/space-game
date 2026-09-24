# Build the Rust WebSocket game server
FROM rust:1.82-slim AS builder

# Keep peak memory low on small build machines
ENV CARGO_BUILD_JOBS=2 \
    RUSTFLAGS="-C debuginfo=0"

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

WORKDIR /app/rust
COPY --from=builder /app/rust/target/release/rust /app/rust/voidrunner
COPY --from=builder /app/client /app/client

ENV PORT=10000
EXPOSE 10000

CMD ["/app/rust/voidrunner"]
