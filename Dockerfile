FROM rust:latest

RUN useradd -m -u 1000 rust
RUN rustup component add rustfmt
USER rust