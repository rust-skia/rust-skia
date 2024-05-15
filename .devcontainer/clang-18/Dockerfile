# This container was created because we need to link with libstdc++ 6.0.25 because of ABI incompatibilities.
# <https://github.com/rust-skia/rust-skia/issues/393>
FROM ubuntu:20.04
# For github actions we need a newer git version
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

# Install Android NDK

# ENV ANDROID_NDK_VERSION=r25b
# ENV ANDROID_HOST=linux-x86_64
# ENV ANDROID_CLANG_VER=14.0.6

# # We use HTTP/1.1, because of spurious HTTP/2 framing errors in the Azure / GitHub cloud.

# RUN curl --http1.1 -sSf -o android-ndk.zip https://dl.google.com/android/repository/android-ndk-${ANDROID_NDK_VERSION}-linux.zip \
#  && unzip android-ndk.zip \
#  && rm android-ndk.zip

# ENV ANDROID_NDK=/android-ndk-${ANDROID_NDK_VERSION}
# ENV PATH=${PATH}:/android-ndk-${ANDROID_NDK_VERSION}/toolchains/llvm/prebuilt/${ANDROID_HOST}/bin

# # <https://github.com/rust-windowing/android-ndk-rs/issues/149>

# ENV ANDROID_NDK_LIB_LINUX=${ANDROID_NDK}/toolchains/llvm/prebuilt/${ANDROID_HOST}/lib64/clang/${ANDROID_CLANG_VER}/lib/linux
# RUN cd ${ANDROID_NDK_LIB_LINUX}/aarch64 && cp libunwind.a libgcc.a
# RUN cd ${ANDROID_NDK_LIB_LINUX}/arm && cp libunwind.a libgcc.a
# RUN cd ${ANDROID_NDK_LIB_LINUX}/i386 && cp libunwind.a libgcc.a
# RUN cd ${ANDROID_NDK_LIB_LINUX}/x86_64 && cp libunwind.a libgcc.a

# wasm32-unknown-emscripten

SHELL ["/bin/bash", "-c"]

RUN apt-get install -y ninja-build

# ENV EMSCRIPTEN_VER=3.1.59

# RUN git clone https://github.com/emscripten-core/emsdk.git \
# 	&& (cd emsdk && ./emsdk install ${EMSCRIPTEN_VER}) \
# 	&& (cd emsdk && ./emsdk activate ${EMSCRIPTEN_VER})
