FROM rust:1.91-slim-bookworm as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl
RUN cargo install dioxus-cli --version 0.7.2
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .

RUN dx build --release --features web --platform web
RUN cargo build --release --features server --no-default-features

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/website /app/server
COPY --from=builder /app/target/dx/website/release/web/public /app/public

ENV PORT=8964
ENV IP=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8964

CMD ["/app/server"]
