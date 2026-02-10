# syntax=docker/dockerfile:1

FROM rust:1.92-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /repo

# Copy only what this crate needs (keeps build context small with a good .dockerignore).
COPY crates/bindings ./crates/bindings
COPY crates/indexer-client ./crates/indexer-client
COPY crates/tron ./crates/tron
COPY crates/aa ./crates/aa
COPY crates/rpc-fallback ./crates/rpc-fallback
COPY crates/observability ./crates/observability
COPY apps/relayer ./apps/relayer

WORKDIR /repo/apps/relayer

# Build with all available cores by default.
# Set `--build-arg CARGO_BUILD_JOBS=<n>` to override when needed.
ARG CARGO_BUILD_JOBS
RUN if [ -n "$CARGO_BUILD_JOBS" ]; then \
      cargo build --release -j "$CARGO_BUILD_JOBS" --bin relayer; \
    else \
      cargo build --release -j "$(nproc)" --bin relayer; \
    fi

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /repo/apps/relayer/target/release/relayer /usr/local/bin/relayer

ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/relayer"]
