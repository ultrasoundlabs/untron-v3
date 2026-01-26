# syntax=docker/dockerfile:1

FROM rust:1.92-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /repo

# Copy only what this crate needs (keeps build context small with a good .dockerignore).
COPY crates/aa ./crates/aa
COPY crates/bindings ./crates/bindings
COPY crates/indexer-client ./crates/indexer-client
COPY crates/aa ./crates/aa
COPY crates/tron ./crates/tron
COPY crates/observability ./crates/observability
COPY apps/realtor ./apps/realtor

WORKDIR /repo/apps/realtor

RUN cargo build --release --bin realtor

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /repo/apps/realtor/target/release/realtor /usr/local/bin/realtor

ENV RUST_LOG=info
# rebuild trigger: observability OTLP/HTTP rollout (v2)

ENTRYPOINT ["/usr/local/bin/realtor"]
