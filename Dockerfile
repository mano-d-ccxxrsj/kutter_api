FROM rust:1.91-slim-bookworm AS chef
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --version 0.1.77 --locked
WORKDIR /usr/src/kutter_api

ENV CARGO_TARGET_DIR=/usr/src/kutter_api/z_target

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/kutter_api/recipe.json recipe.json
RUN RUSTC_WRAPPER="" cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN RUSTC_WRAPPER="" cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1001 appgroup \
    && useradd --uid 1001 --gid appgroup --no-create-home appuser

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/kutter_api/z_target/release/app ./app

RUN chmod 500 ./app && chown appuser:appgroup ./app

USER appuser

CMD ["./app"]