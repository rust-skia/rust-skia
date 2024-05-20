FROM ubuntu:20.04
# For GitHub actions a newer git version is needed
RUN apt-get update \
	&& apt-get install -y wget software-properties-common \
	&& add-apt-repository ppa:git-core/ppa -y

# Download and execute llvm.sh script to install LLVM 18
RUN wget https://apt.llvm.org/llvm.sh \
	&& chmod +x llvm.sh \
	&& ./llvm.sh 18

# unzip: for installing the Android NDK
# libgl1 libgl1-mesa-dev mesa-common-dev: for builds that need OpenGL
# libgles2-mesa-dev for egl support.
RUN apt-get update && apt-get install -y \
	curl \
	gcc \
	g++-18 \
	git \
	libgl1 \
	libgl1-mesa-dev \
	libssl-dev \
	libfontconfig1-dev \
	mesa-common-dev \
	pkg-config \
	python \
	unzip \
	libgles2-mesa-dev
RUN update-alternatives --install /usr/bin/clang clang /usr/bin/clang-18 90 --slave /usr/bin/clang++ clang++ /usr/bin/clang++-18 --slave /usr/bin/llvm-config llvm-config /usr/bin/llvm-config-18
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

SHELL ["/bin/bash", "-c"]

RUN apt-get install -y ninja-build
