FROM alpine:3.15

ENV RUSTFLAGS="-C target-feature=-crt-static" \
    CC="clang" \
    CXX="clang++" \
    GN_EXE="gn" \
    SKIA_GN_COMMAND="/usr/bin/gn" \
    SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN apk update && apk add --update --no-cache \
    curl git python3 python2 clang llvm \
    musl-dev build-base openssl-dev g++ \
    fontconfig-dev fontconfig ttf-dejavu

RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing \
    gn ninja

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

#
# the rest of the file is just to test that the build succeeds (remove before merging)
#
WORKDIR /code

RUN git clone --depth 1 --branch "0.50.0" https://github.com/rust-skia/rust-skia.git && \
    cd rust-skia && \
    git submodule update --init --depth 1 skia-bindings/skia && \
    git submodule update --init --depth 1 skia-bindings/depot_tools

RUN cd rust-skia && \
    cargo build --features=embed-freetype