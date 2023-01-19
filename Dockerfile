FROM debian:bullseye

RUN apt update; \
    apt install -y build-essential curl; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y; \
    . "$HOME/.cargo/env"; \
    rustup toolchain install nightly --allow-downgrade --profile minimal --component rust-src; \
    cargo install cargo-deb
