FROM debian:stretch-backports

RUN apt-get update

# Preinstall tzdata, so that it does not when installed as a transitive dependency later.
ENV TZ=Europe/Berlin
RUN DEBIAN_FRONTEND=noninteractive apt-get install tzdata

# `libgl1-mesa-dev` `mesa-common-dev`: for builds that need OpenGL
# `libgles2-mesa-dev` for egl support.
# `ninja.build` for the ninja build system Skia uses.
# `clang` for the binding generator.
RUN apt-get install -y \
	clang-7 \
	gcc \
	g++ \
	curl \
	git \
	libfontconfig1-dev \
	libssl-dev \
	libgl1-mesa-dev \
	mesa-common-dev \
	pkg-config \
	python \
	ninja.build

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN update-alternatives --install /usr/bin/clang clang /usr/bin/clang-7 90
RUN update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-7 90

COPY . /rust-skia/
WORKDIR /rust-skia/

ENV SKIA_NINJA_COMMAND=/usr/bin/ninja

# RUN cargo run -vv --features "gl,textlayout,embed-freetype"
