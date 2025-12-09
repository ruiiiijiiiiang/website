FROM --platform=$BUILDPLATFORM rust:1.91-slim-bookworm as builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  gcc-aarch64-linux-gnu \
  libc6-dev-arm64-cross \
  perl \
  make \
  gcc \
  && rm -rf /var/lib/apt/lists/*
RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

WORKDIR /app
COPY . .

RUN dx build --release --features web --platform web
RUN cargo build --release --target aarch64-unknown-linux-gnu --features server --no-default-features

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/dx/website/release/web/public /app/public
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/website /app/server

ENV PORT=8964
ENV IP=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8964

CMD ["/app/server"]
