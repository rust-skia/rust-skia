FROM voidlinux/voidlinux-musl

RUN xbps-install -y -Suv
RUN xbps-install -y curl bash gcc llvm clang python git fontconfig-devel
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
WORKDIR /rust-skia/
COPY . /rust-skia/

