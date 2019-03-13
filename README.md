This repository contains safe Rust bindings to the Skia C++ API.

[![Build Status](https://dev.azure.com/armin0390/armin/_apis/build/status/rust-skia.rust-skia?branchName=master)](https://dev.azure.com/armin0390/armin/_build/latest?definitionId=2&branchName=master)

## Goals

We try to provide safe bindings that bridge between Skia's API and idiomatic Rust on all major desktop, mobile, and WebAssembly platforms, including GPU support for Vulkan and OpenGL.

## Building

`cargo build`

Just kidding, we wish it were that simple. Currently you need _at least_ to install LLVM, ninja, and OpenGL libraries.

Please share your experience so that we can complete this section here and try to automate the build to get to the point where `cargo build` _is_ sufficient to build the bindings _including_ Skia, and if that is not possible, clearly prompts to what's missing.

To simplify and speed up the build, we plan to provide prebuilt binaries for some of the major platforms.

## Examples

The examples are taken from Skia's website and [ported to the Rust API](skia-safe/examples/skia-org).

If you were able to build the project run

`cargo run --example skia-org [OUTPUT_DIR]` 

to generate some Skia drawn PNG images in the directory `OUTPUT_DIR`.

## Status

### Crate

Due to the size and it's build requirements of Skia, we'd like to experiment first with prebuilt binaries before releasing a crate.

### Supported Platforms

- [x] Windows
- [x] Linux Ubuntu 16 (18 should work, too).
- [x] MacOSX
- [ ] WebAssembly: #42.
- [ ] Android
- [ ] iOS

### Bindings

Skia is a large library. While we strife to implement a complete set of bindings, it's nowhere complete yet. 

We do support most of the SkCanvas, SkPaint, and SkPath and related APIs and are currently in the process of getting the examples from the [skia.org](https://skia.org/) website to work. Upcoming are the bindings for the classes in the [`include/effects/`](https://github.com/google/skia/tree/2c36ee834ae04d036363cd3b8f3f33ec65d657f0/include/effects) directory.

### Features

- [x] Primitives: Matrix, Rect, Point, Size, etc.
- [x] Basic Drawing: Surface, Canvas, Paint, Path.
- [ ] Effects and Shaders
- [ ] PDF
- [ ] SVG
- [ ] XPS
- [ ] Animation
- [x] Vulkan (rudimentary, basic texture drawing support, build with the cargo feature "vulkan").
- [ ] OpenGL

## This project needs contributions!

If you'd like help with the bindings, take a look at the [Wiki](https://github.com/rust-skia/rust-skia/wiki) to get started and create an issue to avoid duplicate work. For smaller tasks, grep for "TODO" in the source code. And if you want to help making the Rust API nicer, look out for open issues with the label [api conventions](https://github.com/rust-skia/rust-skia/issues?q=is%3Aissue+is%3Aopen+label%3A%22api+conventions%22).

## Maintainers

- LongYinan (@Brooooooklyn)
- Armin (@pragmatrix)

## License

MIT

