FROM lukemathwalker/cargo-chef:latest-rust-1.82.0-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --workspace

FROM debian:bookworm-slim AS runtime
WORKDIR /

COPY --from=builder /app/target/release/cicero-cli cicero-cli
COPY --from=builder /app/target/release/healthcheck healthcheck

RUN useradd -u 1001 cicero
USER cicero

ENTRYPOINT ["/cicero-cli"]