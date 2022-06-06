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

# fake the <asm/hwcap.h> include used by "skia/third_party/externals/zlib/cpu_features.c"
# which doesn't exist on aarch64-alpine but is requested anyway due to an `ifdef ARMV8_OS_LINUX`
RUN mkdir -p /usr/include/c++/10.3.1/aarch64-alpine-linux-musl/asm && \
    touch /usr/include/c++/10.3.1/aarch64-alpine-linux-musl/asm/hwcap.h

# WORKDIR /code

# COPY . /code/rust-skia/

# RUN cd rust-skia && \
#     cargo build --features=embed-freetype
