FROM rust:latest

RUN apt-get update && \
  apt-get install \
    apt-transport-https \
    curl \
    ca-certificates \
    gnupg2 \
    vim \
    software-properties-common \
    musl-tools -y && \
  rustup toolchain add nightly && \
  rustup target add x86_64-unknown-linux-musl --toolchain=nightly && \
  rustup component add rustfmt-preview --toolchain nightly && \
  cargo install cargo-watch
