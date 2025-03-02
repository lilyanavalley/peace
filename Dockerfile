FROM rust:slim
WORKDIR /peace
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN rustup toolchain install wasm32-unknown-unknown
RUN cargo binstall trunk
ENV RUST_LOG=warn
CMD ["trunk", "serve"]
