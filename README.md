# Safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

[![Build Status](https://dev.azure.com/pragmatrix-github/rust-skia/_apis/build/status/rust-skia.rust-skia?branchName=master)](https://dev.azure.com/pragmatrix-github/rust-skia/_build/latest?definitionId=2&branchName=master)

Skia Submodule Status: chrome/m73 ([pending changes][skiapending]).

[skiapending]: https://github.com/google/skia/compare/2c36ee834ae04d036363cd3b8f3f33ec65d657f0...chrome/m73

## Goals

This project attempts to provide _up to date_ safe bindings that bridge idiomatic Rust with Skia's C++ API on all major desktop, mobile, and [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) platforms, including GPU rendering support for [Vulkan](https://en.wikipedia.org/wiki/Vulkan_(API)), [Metal](https://en.wikipedia.org/wiki/Metal_(API)), and [OpenGL](https://en.wikipedia.org/wiki/OpenGL).

## Building

Note that the information in this section is preliminary. Please open an issue for any build problem.

This project requires LLVM, python, and git to build.

To test if LLVM is installed with the correct version, use `clang --version`. Currently, version 7.0.1 is required, or - on Mac OS X - Apple LLVM Version 10 should do, too.

For python, at least version 2.7 should be available. Use `python --version` to see what's there.

### Mac OS X

- Install either Apple LLVM (Version 10) via `xcode-select --install`, or LLVM 7.0.1 via `brew install llvm@7`.

### Windows

- Be sure the `git` command line tool is installed.
- Install the [official LLVM 7.0.1](http://releases.llvm.org/download.html) distribution.
- msys:
  - Install one of the Python2 packages, for example `mingw-w64-x86_64-python2`.
  - LLVM is _always_ picked up from `C:/Program Files/LLVM`, so be sure it's available from there.
- without msys:
  - Download and install Python version 2 from [python.org](https://www.python.org/downloads/release/python-2716/).

### Linux

- LLVM should be installed out of the box, if not, install version 7.0.1.

Then use:

`cargo build -vv`

Under Linux, OpenGL libraries _may_ be missing, if that is the case, install OpenGL drivers for you graphics card, or install a mesa OpenGL package like `libgl1-mesa-dev`.

Please share your build experience so that we can try to automate the build and get to the point where `cargo build` _is_ sufficient to build the bindings _including_ Skia, and if that is not possible, clearly prompts to what's missing.

To simplify and speed up the build, we also plan to provide prebuilt binaries for some of the major platforms ([#49](https://github.com/rust-skia/rust-skia/issues/49)).

## Examples

The examples are taken from [Skia's website](https://skia.org/) and [ported to the Rust API](skia-safe/examples/skia-org).

If you were able to build the project, run

`cargo run --example skia-org -- [OUTPUT_DIR]` 

to generate some Skia drawn PNG images in the directory `OUTPUT_DIR`. To render with OpenGL, use

`cargo run --example skia-org -- [OUTPUT_DIR] --driver opengl`

## Status

### Crate

An official crate is not yet available. We've created [a Milestone](https://github.com/rust-skia/rust-skia/milestone/1) on Github's issue tracker to track the progress.

### Platforms

- [x] Windows
- [x] Linux Ubuntu 16 (18 should work, too).
- [x] MacOSX
- [ ] WebAssembly: [#42](https://github.com/rust-skia/rust-skia/pull/42) (help wanted).
- [ ] Android
- [ ] iOS

### Bindings

Skia is a large library. While we strive to bind all of the C++ APIs, it's nowhere complete yet. 

We do support most of the SkCanvas, SkPaint, and SkPath and related APIs and are trying to make the examples from the [skia.org](https://skia.org/) website work. Recently we merged some of the bindings for the classes in the [`include/effects/`](https://github.com/google/skia/tree/2c36ee834ae04d036363cd3b8f3f33ec65d657f0/include/effects) directory.

### Features

- [x] Vector Graphics: Matrix, Rect, Point, Size, etc.
- [x] Basic Drawing: Surface, Canvas, Paint, Path.
- [x] Basic Effects and Shaders.
- [ ] PDF
- [ ] SVG
- [ ] XPS
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

  