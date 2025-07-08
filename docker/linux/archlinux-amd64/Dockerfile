FROM --platform=linux/amd64 archlinux:latest

ENV RUSTFLAGS="-C target-feature=-crt-static" \
    CC="clang" \
    CXX="clang++" \
    GN_EXE="gn" \
    SKIA_GN_COMMAND="/usr/bin/gn" \
    SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN pacman -Syu --noconfirm \
    curl git python clang llvm \
    musl openssl gcc make \
    fontconfig ttf-dejavu \
    gn ninja

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

# WORKDIR /code

# COPY . /code/rust-skia/

# RUN cd rust-skia && \
#     cargo build -vv -p skia-safe --features=embed-freetype,textlayout
