FROM rust:1.98-bookworm AS builder
WORKDIR /app

COPY ./ ./server

WORKDIR /app/server
RUN cargo build --release


# Runtime stage (slim)
FROM debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 10001 appuser

WORKDIR /app
COPY --from=builder /app/server/target/release/btc-game-server /usr/local/bin/btc-game-server

USER appuser
EXPOSE 9000

CMD ["/usr/local/bin/btc-game-server"]
