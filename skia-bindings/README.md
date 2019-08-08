# Skia Bindings

This is a supporting package for [skia-safe](https:://crates.io/crate/skia-safe), which provides safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

## Organization

This package contains three components. 

- First, full configuration and build support for Skia in [`build.rs`](build.rs) and  [`build_support/`](build_support/).
- Additional C bindings to help out bindgen with stuff it has problems with or to work around linker errors. These are [`src/bindings.cpp`](src/bindings.cpp), and [`src/shaper.cpp`](src/shaper.cpp).
- And a number of functions that are used to download prebuilt binaries.

### Skia Build Support

Building Skia is quite exceptional, a number of prerequisites need to be available and configured properly for the target platform.

To configure and build Skia, [`build_support/skia.rs`](build_support/skia.rs) does all the hard work: it pulls `depot_tools/` and `skia/` from Google's repositories and a number of additional dependencies with the help of Python. After that, it configures Skia with Google's [GN](https://gn.googlesource.com/gn/+/refs/heads/master/README.md) tool, and finally builds it by giving control to the `ninja` executable from the `depot_tools/` package.

### Binding Generation

`src/bindings.rs` and `src/shaper.cpp` contain the C++ code that Rust needs to interact with Skia's codebase. These files are processed by the [Rust's binding generator](<https://github.com/rust-lang/rust-bindgen>) that uses libclang for the layout computation _and also_ compiled by [clang](https://clang.llvm.org/).

If both steps went well, the resulting Rust binding code is written to `src/bindings.rs`, and the `skia-bindings` library is found in the output directory alongside where Skia was built previously.

### Prebuilt Binaries

Because building Skia _and_ creating the bindings is slow and depend on a number of components that lie outside the Rust ecosystem, we decided to experiment with prebuilt binaries.

Whenever a new version of `rust-skia` is built from the `release` branch on our CI server, the resulting Skia library, `skia-bindings` library, _and_ `bindings.rs` are uploaded to the releases tab of the [skia-binaries repository](<https://github.com/rust-skia/skia-binaries/releases>).

And whenever the build script detects that `skia-bindings` is built from inside a crate _and_ a prebuilt archive is available that matches the repository's hash, platform, and features, it downloads the package, unpacks it, and skips the full build step of Skia and the bindings.

## Build Customization

Besides of the features `vulkan`, `svg`, and `shaper` that can be directly specified when the package is added as a cargo dependency, the Skia build can be customized further in `build.rs` by adjusting one of two structs that are defined in `build_support/skia.rs`:

### `BuildConfiguration`

This struct represents the top level build configuration for `skia-bindings` and contains a number of individual feature flags.

### `FinalBuildConfiguration`

The `FinalBuildConfiguration` is created from the `BuildConfiguration` and contains name value pairs used by GN to parameterize the Skia build and preprocessor defines used to create the `src/bindings.rs` file and the `skia-bindings` library.

