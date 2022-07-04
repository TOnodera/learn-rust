FROM rust:latest

RUN useradd -m -u 1000 rust
USER rust