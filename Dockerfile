FROM rust:latest AS builder

WORKDIR user/src/kutter_api

COPY Cargo.toml ./Cargo.toml
COPY app ./app
COPY infra ./infra
COPY persistence ./persistence
COPY security ./security
COPY shared ./shared
COPY web ./web

RUN cargo fetch

COPY ./ ./

RUN cargo build --release

FROM debian:latest

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/kutter_api/z_target/release/ .

COPY .env ./

CMD ["./app"]