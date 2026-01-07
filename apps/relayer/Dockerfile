# syntax=docker/dockerfile:1

FROM rust:1.92-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /repo

# Copy only what this crate needs (keeps build context small with a good .dockerignore).
COPY crates/bindings ./crates/bindings
COPY crates/indexer-client ./crates/indexer-client
COPY crates/tron ./crates/tron
COPY apps/relayer ./apps/relayer

WORKDIR /repo/apps/relayer

RUN cargo build --release --bin relayer

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /repo/apps/relayer/target/release/relayer /usr/local/bin/relayer

ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/relayer"]
