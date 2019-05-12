# Skia Bindings

This package contains build support for Skia and number of additional C functions that support the package skia-safe.

## Organization

This package has three primary parts. Build support for Skia, the source for the generated Rust bindings `src/bindings.cpp`, and a number of functions that are used to manage prebuilt binaries.

### Skia Build Support

Building Skia is quite exceptional, a number of prerequisites need to be available and configured properly for the platform it is built for. 

To configure and build Skia, `build_support/skia.rs` does all the hard work: it pulls `depot_tools/` and `skia/` from Google's repositories and a number of additional dependencies with Python. After that, it configures Skia with [GN](<https://chromium.googlesource.com/chromium/src/tools/gn/+/48062805e19b4697c5fbd926dc649c78b6aaa138/README.md>), and finally builds it by giving control to the `ninja` executable from the `depot_tools/` package.

### Binding Generation

`src/bindings.cpp` contains the C++ code that Rust needs to speak with Skia's codebase. This file is processed by the [Rust's binding generator](<https://github.com/rust-lang/rust-bindgen>) _and_ compiled by clang.

If both went well, the Rust binding code is available at `src/bindings.rs`, and the `skia-bindings` library is found in the output directory alongside where Skia was built previously. 

### Prebuilt Binaries

Because the Skia build _and_ the binding generation is quite complicated and depends on a number of factors that lie outside the Rust ecosystem, we decided to experiment with prebuilt binaries.

Whenever a new version of `rust-skia` is built from the `master` branch on our CI server, the resulting Skia library, `skia-bindings` library, _and_ `bindings.rs` are uploaded to the releases tab of the [skia-binaries repository](<https://github.com/rust-skia/skia-binaries/releases>).

And whenever the build script detects that `skia-bindings` is built from inside a crate _and_ a prebuilt archive is available that matches the repository's hash, platform, and features, it downloads the package, unpacks it, and skips the full build step of Skia and the bindings.

## Build Customization

Besides of the features `vulkan` and `svg` that can be directly specified when the package is added as a cargo dependency, the Skia build can be further customized in `build.rs` by adjusting one of two structs that are defined in `src/build_support/skia.rs`:

### `BuildConfiguration`

This struct represents the top level build configuration for `skia-bindings` and contains a number of individual feature flags.

### `FinalBuildConfiguration`

The `FinalBuildConfiguration` is created from the `BuildConfiguration` and contains name value pairs used by GN to parameterize the Skia build and preprocessor defines used to create the `src/bindings.rs` file and the `skia-bindings` library.

