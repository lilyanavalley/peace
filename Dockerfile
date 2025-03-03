FROM rust:latest
WORKDIR /peace
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
# install binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# install trunk
RUN cargo binstall trunk
# RUN cargo install trunk
# ENV RUST_LOG=warn
ENV TRUNK_SERVE_ADDRESS=0.0.0.0
ENV TRUNK_BUILD_RELEASE=true
EXPOSE 8080
CMD ["trunk", "serve", "-v"]
