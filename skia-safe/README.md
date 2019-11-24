# <img alt="" width="48" align="top"  src="https://raw.githubusercontent.com/rust-skia/rust-skia/master/artwork/rust-skia-icon_512x512.png"/> Safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

This packages contains safe Rust wrappers for Skia and uses [skia-bindings](https://crates.io/crates/skia-bindings) to build and interface with the Skia C++ library.

For information about the supported build targets and how to run the examples, please visit the [github page of the rust-skia project](https://github.com/rust-skia/rust-skia).

## Documentation

Function level documentation is [not yet](https://github.com/rust-skia/rust-skia/issues/23) available. To get started, take a look at the [Rust examples](https://github.com/rust-skia/rust-skia/tree/master/skia-safe/examples/skia-org) or the [Skia documentation](https://skia.org). 

## Bindings & Wrappers

Skia-safe wraps most parts of the public Skia C++ APIs:

- [x] Vector Geometry: Matrix, Rect, Point, Size, etc.
- [x] Most drawing related classes and functions: Surface, Canvas, Paint, Path.
- [x] Effects and Shaders.
- [x] Utility classes we think are useful.
- [x] PDF & SVG rendering
- [ ] Skia Modules
  - [x] Text shaping with [Harfbuzz](https://www.freedesktop.org/wiki/Software/HarfBuzz/) and [ICU](http://site.icu-project.org/home).
  - [x] Text layout (skparagraph)
  - [ ] Animation via [Skottie](https://skia.org/user/modules/skottie)
- [ ] GPU Backends
  - [x] Vulkan
  - [x] OpenGL
  - [ ] Metal

Wrappers for functions that take callbacks and virtual classes are not supported right now. While we think they should be wrapped, the use cases related seem to be rather special, so we postponed that for now.

## Features

Skia-safe supports the following features that can be configured [via cargo](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section):

### `vulkan`

Vulkan support can be enabled by setting the Cargo feature `default = ["vulkan"]` in `skia-safe/Cargo.toml`, which will cause a rebuild of Skia. To render the examples with Vulkan use `cargo run --example skia-org -- [OUTPUT_DIR] --driver vulkan`.

Note that Vulkan drivers need to be available. On Windows, they are most likely available already, on Linux [this article on linuxconfig.org](<https://linuxconfig.org/install-and-test-vulkan-on-linux>) might get you started, and on macOS with Metal support, [install the Vulkan SDK](<https://vulkan.lunarg.com/sdk/home>) for Mac and configure MoltenVK by setting the `DYLD_LIBRARY_PATH`, `VK_LAYER_PATH`, and `VK_ICD_FILENAMES` environment variables as described in `Documentation/getting_started_macos.html`.

### `svg`

This feature enables the SVG rendering backend. To create a new Skia canvas that renders to SVG, use the function `skia-safe::svg::Canvas::new()`.

### `shaper`

The Cargo feature `shaper` enables text shaping with Harfbuzz and ICU. 

On **Windows**, the file `icudtl.dat` must be available in your executable's directory. To provide the data file, either copy it from the build's output directory (shown when skia-bindings is compiled with `cargo build -vv | grep "ninja: Entering directory"`), or - if your executable directory is writable - invoke the function `skia_safe::icu::init()` before creating the `skia_safe::Shaper` object. 

A simple example can be found [in the skia-org command line application](https://github.com/rust-skia/rust-skia/blob/master/skia-safe/examples/skia-org/skshaper_example.rs).

### `textlayout`

This feature makes the Skia module skparagraph available, which contains types that are used to lay out paragraphs. In Rust, the types are available from the `skia_safe::textlayout` module. 

A code snippet that lays out a paragraph can be found [in the skia-org example](https://github.com/rust-skia/rust-skia/blob/master/skia-safe/examples/skia-org/skshaper_example.rs).

