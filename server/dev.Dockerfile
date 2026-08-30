FROM rust:1.98-bookworm

# Dev tooling
RUN cargo install cargo-watch

WORKDIR /app/server

EXPOSE 9000

CMD ["cargo", "watch", "-x", "run"]
