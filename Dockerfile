FROM rust:1.91-bookworm AS builder

RUN apt-get update &&\
  apt-get install -y \
  binaryen \
  libssl-dev \
  make \
  perl \
  pkg-config && \
  rm -rf /var/lib/apt/lists/* &&\
  cargo install dioxus-cli &&\
  rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN echo '#!/bin/sh\nexit 0' > /usr/local/bin/strip && chmod +x /usr/local/bin/strip
RUN dx build --release

FROM debian:bookworm-slim

RUN apt-get update && \
  apt-get install -y libssl3 ca-certificates && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/dx/website/release/web /app

ENV PORT=8964
ENV IP=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8964

CMD ["/app/website"]
