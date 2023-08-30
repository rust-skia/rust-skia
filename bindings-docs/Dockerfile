FROM ghcr.io/pragmatrix/rust-skia-linux:latest

WORKDIR /rust-skia/
COPY . /rust-skia/

RUN cargo build --features "gl,textlayout,webp"
RUN cp `make locate-bindings` /tmp/bindings.rs
