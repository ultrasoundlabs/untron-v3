# syntax=docker/dockerfile:1

FROM rust:1.92-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /repo

# Copy only what this crate needs (keeps build context small with a good .dockerignore).
COPY crates/tron ./crates/tron
COPY crates/observability ./crates/observability
COPY apps/pool ./apps/pool

WORKDIR /repo/apps/pool

RUN cargo build --release --bin pool

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /repo/apps/pool/target/release/pool /usr/local/bin/pool

ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/pool"]

