# Rust nightly
FROM rustlang/rust:nightly-bookworm AS builder

ENV RUST_BACKTRACE=full

# Install build dependencies
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends build-essential clang pkg-config libssl-dev ca-certificates  openssl \
  && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
  && apt-get install -y nodejs
RUN openssl version

# Install binstall
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y

# Install wasm compiler
RUN rustup target add wasm32-unknown-unknown

# Peace build
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN npm i
RUN cargo leptos build --release -vv

# Peace runner
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates curl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/peace /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_BACKTRACE=1
ENV RUST_LOG="warn"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/peace"]
