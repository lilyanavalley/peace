# Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install build dependencies
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends clang

# Install binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos -y

# Install wasm compiler
RUN rustup target add wasm32-unknown-unknown

# Peace build
WORKDIR /peace
COPY . .
RUN cargo leptos build --release -vv

# Peace runner
FROM debian:bookworm-slim as runtime
WORKDIR /peace
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /peace/target/release/peace /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /peace/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /peace/Cargo.toml /app/

ENV RUST_LOG="warn"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/peace"]
