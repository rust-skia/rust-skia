# <img alt="" width="48" align="top" src="artwork/rust-skia-icon_512x512.png"/> Safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

[![crates.io](https://img.shields.io/crates/v/skia-safe)](https://crates.io/crates/skia-safe) [![license](https://img.shields.io/crates/l/skia-safe)](LICENSE) [![Build Status](https://dev.azure.com/pragmatrix-github/rust-skia/_apis/build/status/rust-skia.rust-skia?branchName=master)](https://dev.azure.com/pragmatrix-github/rust-skia/_build/latest?branchName=master)

Skia Submodule Status: chrome/m79 ([pending changes][skiapending]).

[skiapending]: https://github.com/google/skia/compare/2542bdfcd6...chrome/m79

## Goals

This project attempts to provide _up to date_ safe bindings that bridge idiomatic Rust with Skia's C++ API on all major desktop, mobile, and [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) platforms, including GPU rendering support for [Vulkan](https://en.wikipedia.org/wiki/Vulkan_(API)), [Metal](https://en.wikipedia.org/wiki/Metal_(API)), and [OpenGL](https://en.wikipedia.org/wiki/OpenGL).

## Status

### Crate

Although we recommend to use the git repository because the [prerelease on crates.io](https://crates.io/crates/skia-safe) is a bit flaky at the moment, adding

```toml
[dependencies]
skia-safe = "0"
```

to your `Cargo.toml` should get you started.

### Platforms & Build Targets

- [x] Windows
- [x] Linux Ubuntu 18 (16 should work, too).
- [x] macOS
- [x] Android (macOS | Linux -> aarch64, contributed by [@DenisKolodin](https://github.com/DenisKolodin))
- [x] iOS
- [ ] WebAssembly: [#42](https://github.com/rust-skia/rust-skia/pull/42) (help wanted).

### Bindings & Supported Features

The supported bindings and Skia features are described in the [skia-safe package's readme](skia-safe/README.md).

## Building

Note that the information in this section is preliminary. Please open an issue for any build problem.

### Prerequisites

This project requires **LLVM**, **Python 2**, and **OpenSSL libraries** to build.

To see which version of LLVM/Clang is available, use `clang --version`. 

We recommend version 8, but also had successes to build Skia with 6.0.1 and 7.0.1, and - on macOS - Apple LLVM version 11. So it's probably best to use the preinstalled version or install version 8 if LLVM is not available on your platform by default.

Python version 2.7 _must_ be available. The build script probes for `python --version` and `python2 --version` and uses the first one that looks like a version 2 executable.

OpenSSL libraries can be installed on **Debian** and **Ubuntu** with:

```bash
sudo apt-get install pkg-config libssl-dev
```

For other platforms, more information is available at the [OpenSSL crate documentation](https://docs.rs/openssl/0.10.24/openssl/#automatic).

### macOS

- Install the XCode command line developer tools with

  ```bash
  xcode-select --install
  ```

- **macOS Mojave Version 10.14**: install the SDK headers:

  ```bash
  sudo open /Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg
  ```

  otherwise the Skia build _may_ fail to build `SkJpegUtility.cpp` and the binding generation _will_ fail with  `'TargetConditionals.h' file not found` . Also note that the command line developer tools _and_ SDK headers _should_ be reinstalled after an update of XCode.

- As an alternative to Apple LLVM 10, install LLVM via `brew install llvm` or `brew install llvm@7` and then set `PATH`, `CPPFLAGS`, and `LDFLAGS` like instructed.

### Windows

- Have the latest versions of `git` and Rust ready.
- [Install Visual Studio 2019 Build Tools](https://visualstudio.microsoft.com/downloads/) or one of the other IDE releases. If you installed the IDE version, make sure that the [Desktop Development with C++ workload](https://docs.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=vs-2019) is installed.
- Install the [latest LLVM 8](http://releases.llvm.org/download.html) distribution.
- [MSYS2](https://www.msys2.org/):
  - Install Python2 with `pacman -S python2`.
  - `clang` is _always_ picked up from `C:/Program Files/LLVM/bin`, so be sure it's available from there.
- Windows Shell (Cmd.exe):
  
  - Download and install Python version 2 from [python.org](https://www.python.org/downloads/release/python-2716/).
- Install and switch to the MSVC toolchain:
  ```bash
  rustup default stable-msvc
  ```

### Linux

- LLVM/Clang should be installed out of the box, if not, install version 8.

Then use:

```bash
cargo build -vv
```

On Linux, OpenGL libraries _may_ be missing, if that is the case, install OpenGL drivers for you graphics card, or install a mesa OpenGL package like `libgl1-mesa-dev`.

Please share your build experience so that we can try to automate the build and get to the point where `cargo build` _is_ sufficient to build the bindings _including_ Skia, and if that is not possible, clearly prompts to what's missing.

### Android

Cross compilation to Android is supported for targeting 64 bit ARM and Intel x86 architectures (`aarch64` and `x86_64`) for API Level 26 (Oreo, Android 8):

For example, to compile for `aarch64`:

1. Install the rust target:
   ```bash
   rustup target install aarch64-linux-android
   ```
2. Download the [r20 NDK](https://developer.android.com/ndk/downloads) for your host architecture and unzip it.
3. Compile your package for the `aarch64-linux-android` target:

On **macOS**:

```bash
ANDROID_NDK=:path-to-android-ndk-r20 PATH=$PATH:$ANDROID_NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin CC_aarch64_linux_android=aarch64-linux-android26-clang CXX_aarch64_linux_android=aarch64-linux-android26-clang++ CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android26-clang cargo build --target aarch64-linux-android -vv
```

On **Linux**:

```bash
ANDROID_NDK=:path-to-android-ndk-r20 PATH=$PATH:$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin CC_aarch64_linux_android=aarch64-linux-android26-clang CXX_aarch64_linux_android=aarch64-linux-android26-clang++ CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android26-clang cargo build --target aarch64-linux-android -vv
```

On **Windows** the Android NDK clang executable must be invoked through `.cmd` scripts:

```bash
ANDROID_NDK=:path-to-android-ndk-r20 PATH=$PATH:$ANDROID_NDK/toolchains/llvm/prebuilt/windows-x86_64/bin CC_aarch64_linux_android=aarch64-linux-android26-clang.cmd CXX_aarch64_linux_android=aarch64-linux-android26-clang++.cmd CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android26-clang.cmd cargo build --target aarch64-linux-android -vv
```
_Notes:_

- The `CARGO_TARGET_${TARGET}_LINKER` environment variable name [needs to be all uppercase](https://github.com/rust-lang/cargo/issues/1109#issuecomment-386850387).
- In some older shells (for example macOS High Sierra), environment variable replacement can not be used when the variable was defined on the same line. Therefore the `ANDROID_NDK` variable must be defined before it's used in the `PATH` variable.
- Rebuilding skia-bindings with a different target may cause linker errors, in that case `touch skia-bindings/build.rs` will force a rebuild ([#10](https://github.com/rust-skia/rust-skia/issues/10)).

### iOS

Compilation to iOS is supported on macOS targeting the iOS simulator (`--target x86_64-apple-ios`) and 64 bit ARM devices (`--target aarch64-apple-ios`).

### Skia

For situations in which Skia does not build or needs to be configured differently, we support some customization support in `skia-bindings/build.rs`. For more details about how to customize Skia builds, take a look at the [README of the skia-bindings package](skia-bindings/README.md).

Note that crate packages _will_ try to download prebuilt binaries from [skia-binaries](<https://github.com/rust-skia/skia-binaries/releases>) if the platform matches with one of the binaries build on the CI. If the download fails, a full build of Skia is triggered.

## Examples

The `icon` example generates the rust-skia icon in the current directory.
It computes the position of all the gear teeth etc. based on parameters such as the number of teeth and wheel radius.

If you were able to build the project, run

```bash
cargo run --example icon 512
```

It has a single optional parameter which is the size in pixels for the PNG file.
Without parameters, it’ll produce PNG frames for the [animated version](https://matracas.org/tmp/rust-skia-icon.html).

The other examples are taken from [Skia's website](https://skia.org/) and [ported to the Rust API](skia-safe/examples/skia-org).

```bash
cargo run --example skia-org -- [OUTPUT_DIR]
```

to generate some Skia drawn PNG images in the directory `OUTPUT_DIR`. To render with OpenGL, use

```bash
cargo run --example skia-org -- [OUTPUT_DIR] --driver opengl
```

And to show the drivers that are supported
```bash 
cargo run --example skia-org -- --help
```

Some examples:

Fill, Radial Gradients, Stroke, Stroke with Gradient, Transparency:
[![Rust-skia icon](artwork/rust-skia-icon_512x512.png)](https://matracas.org/tmp/rust-skia-icon.html)

Fill, Stroke, Text:

![Fill, Stroke, Text](https://rust-skia.github.io/skia-org/cpu/SkPaint-Overview/02-fill-and-stroke.png)

Sweep Gradient:

![Sweep Gradient](https://rust-skia.github.io/skia-org/cpu/SkPaint-Overview/08-sweep-gradient-shader.png)

Dash Path Effect:

![Dash Path Effect](https://rust-skia.github.io/skia-org/cpu/SkPaint-Overview/19-dash-path-effect.png)

For more, you may take a look at the [rust-skia.github.io](https://github.com/rust-skia/rust-skia.github.io/tree/master/skia-org/cpu) repository.

## This project needs contributions!

If you'd like to help with the bindings, take a look at the [Wiki](https://github.com/rust-skia/rust-skia/wiki) to get started and create an issue to avoid duplicate work. For smaller tasks, grep for "TODO" in the source code. And for heroic work, check out the label [help wanted](https://github.com/rust-skia/rust-skia/labels/help%20wanted). And if you like to help making the Rust API nicer to use, look out for open issues with the label [api ergonomics](https://github.com/rust-skia/rust-skia/issues?q=is%3Aissue+is%3Aopen+label%3A%22api+ergonomics%22).

## Notable Contributions

- The Rust-Skia Logo and the example program that renders it, by Alberto González Palomo ([@AlbertoGP](https://github.com/AlbertoGP))

## Maintainers

- LongYinan ([@Brooooooklyn](https://github.com/Brooooooklyn))
- Armin ([@pragmatrix](https://github.com/pragmatrix))

## License

MIT

  
