# https://github.com/rust-skia/rust-skia/issues/535
FROM arm64v8/ubuntu:focal

RUN apt-get update

# Preinstall tzdata, so that it does not when installed as a transitive dependency later.
ENV TZ=Europe/Berlin
RUN DEBIAN_FRONTEND=noninteractive apt-get install tzdata

# `libgl1` `libgl1-mesa-dev` `mesa-common-dev`: for builds that need OpenGL
# `libgles2-mesa-dev` for egl support.
# `ninja.build` for the ninja build system Skia uses.
# `clang` for the binding generator.
RUN apt-get install -y \
	clang \
	curl \
	g++-9 \
	gcc \
	git \
	libfontconfig1-dev \
	libgl1 \
	libgl1-mesa-dev \
	libgles2-mesa-dev \
	libssl-dev \
	mesa-common-dev \
	pkg-config \
	python \
	ninja.build

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . /rust-skia/
WORKDIR /rust-skia/

RUN cargo build -vv --features "gl"
