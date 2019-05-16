# Safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

[![Build Status](https://dev.azure.com/pragmatrix-github/rust-skia/_apis/build/status/rust-skia.rust-skia?branchName=master)](https://dev.azure.com/pragmatrix-github/rust-skia/_build/latest?definitionId=2&branchName=master)

Skia Submodule Status: chrome/m74 ([pending changes][skiapending]).

[skiapending]: https://github.com/google/skia/compare/ae4b97edd5b9eeee9e4fe9814f67e3abc4ba1a75...chrome/m74

## Goals

This project attempts to provide _up to date_ safe bindings that bridge idiomatic Rust with Skia's C++ API on all major desktop, mobile, and [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) platforms, including GPU rendering support for [Vulkan](https://en.wikipedia.org/wiki/Vulkan_(API)), [Metal](https://en.wikipedia.org/wiki/Metal_(API)), and [OpenGL](https://en.wikipedia.org/wiki/OpenGL).

## Building

Note that the information in this section is preliminary. Please open an issue for any build problem.

### Prerequisites

This project requires LLVM, Python 2, and git to build.

To see which version of LLVM/Clang is available, use `clang --version`. 

We recommend version 8, but also had successes to build Skia with 6.0.1 and 7.0.1, and - on macOS - Apple LLVM version 10. So it's probably best to use the preinstalled version or install version 8 if LLVM is not available on your platform by default.

For Python, at least version 2.7 _should_ be available! Use `python --version` to see what's there.

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

- Be sure the `git` command line tool is installed.
- Install the [official LLVM 8](http://releases.llvm.org/download.html) distribution.
- msys:
  - Install one of the Python2 packages, for example `mingw-w64-x86_64-python2`.
  - LLVM is _always_ picked up from `C:/Program Files/LLVM`, so be sure it's available from there.
- without msys:
  - Download and install Python version 2 from [python.org](https://www.python.org/downloads/release/python-2716/).

### Linux

- LLVM should be installed out of the box, if not, install version 8.

Then use:

`cargo build -vv`

Under Linux, OpenGL libraries _may_ be missing, if that is the case, install OpenGL drivers for you graphics card, or install a mesa OpenGL package like `libgl1-mesa-dev`.

Please share your build experience so that we can try to automate the build and get to the point where `cargo build` _is_ sufficient to build the bindings _including_ Skia, and if that is not possible, clearly prompts to what's missing.

### Skia

For situations in which Skia does not build or needs to be configured differently, we support some customization support in `skia-bindings/build.rs`. For more details about how to customize Skia builds, take a look at the [README of the skia-bindings package](skia-bindings/README.md).

Note that official crate packages _will_ try to download prebuilt binaries from [skia-binaries](<https://github.com/rust-skia/skia-binaries/releases>) if the platform matches with one of our images we test our builds with. If the download fails, a full build of Skia is triggered.

### Feature `vulkan`

Vulkan support can be enabled by setting the Cargo feature `default = ["vulkan"]` in `skia-safe/Cargo.toml`, which will cause a rebuild of Skia. To render the examples with Vulkan use `cargo run --example skia-org -- [OUTPUT_DIR] --driver vulkan`.

Note that Vulkan drivers need to be available. On Windows, they are most likely available already, on Linux [this article on linuxconfig.org](<https://linuxconfig.org/install-and-test-vulkan-on-linux>) might get you started, and on macOS with Metal support, [install the Vulkan SDK](<https://vulkan.lunarg.com/sdk/home>) for Mac and configure MoltenVK by setting the `DYLD_LIBRARY_PATH`, `VK_LAYER_PATH`, and `VK_ICD_FILENAMES` environment variables as described in `Documentation/getting_started_macos.html`.

## Examples

The examples are taken from [Skia's website](https://skia.org/) and [ported to the Rust API](skia-safe/examples/skia-org).

If you were able to build the project, run

`cargo run --example skia-org -- [OUTPUT_DIR]` 

to generate some Skia drawn PNG images in the directory `OUTPUT_DIR`. To render with OpenGL, use

`cargo run --example skia-org -- [OUTPUT_DIR] --driver opengl`

And `cargo run --example skia-org -- --help` shows the drivers that are currently supported.

## Status

### Crate

An official crate is not yet available on [crates.io](<https://crates.io/>) but every update to the master branch releases new crates to the [releases tab](<https://github.com/rust-skia/rust-skia/releases>) and there is also a [a Milestone](https://github.com/rust-skia/rust-skia/milestone/1) to track the progress.

### Platforms

- [x] Windows
- [x] Linux Ubuntu 16 (18 should work, too).
- [x] macOS X
- [ ] WebAssembly: [#42](https://github.com/rust-skia/rust-skia/pull/42) (help wanted).
- [ ] Android
- [ ] iOS

### Bindings

Skia is a large library. While we strive to bind all of the C++ APIs, it's nowhere complete yet. 

We do support most of the SkCanvas, SkPaint, and SkPath and related APIs and are trying to make the examples from the [skia.org](https://skia.org/) website work.

### Features

- [x] Vector Graphics: Matrix, Rect, Point, Size, etc.
- [x] Basic Drawing: Surface, Canvas, Paint, Path.
- [x] Basic Effects and Shaders.
- [x] PDF
- [x] SVG
- [ ] Animation
- [x] Vulkan
- [x] OpenGL
- [ ] Metal

## This project needs contributions!

If you'd like to help with the bindings, take a look at the [Wiki](https://github.com/rust-skia/rust-skia/wiki) to get started and create an issue to avoid duplicate work. For smaller tasks, grep for "TODO" in the source code. And for heroic work, check out the label [help wanted](https://github.com/rust-skia/rust-skia/labels/help%20wanted). And if you like to help making the Rust API nicer to use, look out for open issues with the label [api ergonomics](https://github.com/rust-skia/rust-skia/issues?q=is%3Aissue+is%3Aopen+label%3A%22api+ergonomics%22).

## Maintainers

- LongYinan (@Brooooooklyn)
- Armin (@pragmatrix)

## License

MIT

  
