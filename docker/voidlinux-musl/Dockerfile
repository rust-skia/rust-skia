FROM voidlinux/voidlinux-musl

RUN xbps-install -y -S
RUN xbps-install -y -u xbps
RUN xbps-install -y curl bash gcc llvm clang python git fontconfig-devel ninja
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV GN_COMMIT=dfcbc6fed0a8352696f92d67ccad54048ad182b3

RUN \
  git clone https://gn.googlesource.com/gn /tmp/gn \
  && git -C /tmp/gn checkout ${GN_COMMIT} \
  && cd /tmp/gn \
  && python build/gen.py \
  && ninja -C out \
  && cp -f /tmp/gn/out/gn /usr/local/bin/gn

WORKDIR /rust-skia/
COPY . /rust-skia/
ENV SKIA_GN_COMMAND=gn
ENV SKIA_NINJA_COMMAND=ninja
ENV PATH=$PATH:/root/.cargo/bin
# RUN cargo build -vv

