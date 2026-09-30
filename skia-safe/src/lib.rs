//! Safe Rust bindings for the [Skia Graphics Library](https://skia.org/).
//!
//! This crate provides idiomatic, memory-safe Rust wrappers around Skia's C++ API, so that you can
//! draw text, geometry, and images from Rust on desktop and mobile platforms, including GPU
//! rendering backends for [Vulkan](https://en.wikipedia.org/wiki/Vulkan_(API)),
//! [Metal](https://en.wikipedia.org/wiki/Metal_(API)), [OpenGL](https://en.wikipedia.org/wiki/OpenGL),
//! and [Direct3D](https://en.wikipedia.org/wiki/Direct3D).
//!
//! # Why this crate exists
//!
//! Skia is a large, fast-moving C++ library that is primarily developed for use inside Chromium and
//! Android. One of this project's goals is to stay **up to date with the current Skia milestone
//! releases**: the bindings are regenerated and the wrappers updated for every new milestone.
//!
//! The current milestone is [`MILESTONE`], and the crate version tracks it.
//!
//! # Architecture
//!
//! This crate is the safe layer of a two-crate stack:
//!
//! - [`skia-bindings`](https://crates.io/crates/skia-bindings) builds Skia from source (or downloads
//!   a prebuilt binary) and generates the low-level `extern "C"` bindings with
//!   [bindgen](https://github.com/rust-lang/rust-bindgen).
//! - **`skia-safe`** (this crate) wraps those bindings in safe, Rust-idiomatic types.
//!
//! Most Skia types are re-exported at the crate root (for example [`Canvas`], [`Surface`],
//! [`Paint`], [`Path`], [`Image`]), so you can usually write `skia_safe::Canvas` without digging
//! into the module tree.
//!
//! # Getting started
//!
//! ```toml
//! [dependencies]
//! skia-safe = "0"
//! ```
//!
//! A minimal example that draws into a raster surface and saves a PNG:
//!
//! ```no_run
//! use skia_safe::{Color, EncodedImageFormat, ImageInfo, Paint, Surface};
//!
//! fn main() {
//!     let mut surface = Surface::new_raster_n32_premul((256, 256)).unwrap();
//!     let canvas = surface.canvas();
//!     canvas.clear(Color::WHITE);
//!     let mut paint = Paint::default();
//!     paint.set_anti_alias(true);
//!     canvas.draw_circle((128.0, 128.0), 100.0, &paint);
//!
//!     let image = surface.image_snapshot();
//!     let png = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
//!     std::fs::write("out.png", png.as_bytes()).unwrap();
//! }
//! ```
//!
//! # Module overview
//!
//! The crate mirrors Skia's own header layout:
//!
//! - `core` — the core drawing types: [`Surface`], [`Paint`], [`Path`],
//!   [`Image`], [`Font`], [`Matrix`], [`Color`], and friends.
//! - [`gpu`] — GPU support: the Ganesh ([`gpu::ganesh`]) and Graphite backends and their
//!   backend-specific APIs (D3D, GL, Metal, Vulkan).
//! - `effects` — shaders, color filters, image filters, path effects, and runtime
//!   effects (SkSL).
//! - [`codec`] — decoding of encoded images into [`Image`]s and [`Pixmap`]s.
//! - `encode_` — encoding of images into the supported
//!   [`EncodedImageFormat`]s.
//! - `pathops` — boolean operations on [`Path`]s.
//! - [`svg`] — SVG rendering.
//! - [`skottie`] — Lottie (bodymovin) animation rendering.
//! - [`textlayout`] — multi-line, styled text layout via Skia's `skparagraph`
//!   module (enabled by the `textlayout` feature).
//! - [`shapers`] — text shaping backends built on Skia's `SkShaper`.
//! - [`utils`] — miscellaneous helpers: path parsing, text drawing, shadows, cameras,
//!   and typefaces.
//! - [`wrapper`] — FFI interoperability traits for the wrapper types.
//!
//! # Features
//!
//! By default the crate is configured for CPU (raster) rendering only. GPU backends and optional
//! modules are enabled through Cargo features:
//!
//! - `gl` — OpenGL / OpenGL ES (implies Ganesh). On Linux, `egl`, `x11`, and `wayland` configure
//!   window-manager integration.
//! - `vulkan` — Vulkan support (requires `ganesh` or `graphite`).
//! - `metal` — Metal support on macOS and iOS (requires `ganesh` or `graphite`).
//! - `d3d` — Direct3D support on Windows.
//! - `textlayout` — text shaping with HarfBuzz and ICU, and text layout via `skparagraph`.
//! - `svg` — SVG rendering.
//! - `skottie` — Lottie animation rendering.
//! - `webp-encode`, `webp-decode`, `webp` — WebP image format support.
//! - `binary-cache` (default) — download prebuilt Skia binaries instead of building locally.
//! - `embed-icudtl` (default) — embed `icudtl.dat` for the `textlayout` features.
//!
//! See the [package README](https://github.com/rust-skia/rust-skia/blob/master/skia-safe/README.md)
//! for the full list of supported wrappers, codecs, and features.

#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::non_send_fields_in_send_ty)]
// https://github.com/rust-lang/rust/issues/93367
#![allow(unknown_lints)]
#![allow(clippy::too_long_first_doc_paragraph)]
#![allow(clippy::doc_overindented_list_items)]
#![allow(mismatched_lifetime_syntaxes)]

#[cfg(feature = "gpu")]
compile_error!(
    "feature `gpu` has been renamed to `ganesh`; replace `gpu` with `ganesh`. The `vulkan` and `metal` features require either `ganesh` or `graphite`."
);

#[cfg(all(
    any(feature = "vulkan", feature = "metal"),
    not(any(feature = "ganesh", feature = "graphite"))
))]
compile_error!(
    "the `vulkan` and `metal` features require at least one rendering engine: `ganesh` or `graphite`"
);

mod macros;

pub mod codec;
#[deprecated(since = "0.33.1", note = "use codec::Result")]
pub use codec::Result as CodecResult;
pub use codec::{Codec, EncodedImageFormat, EncodedOrigin, codecs};

mod core;
pub mod cpu;
#[cfg(feature = "pdf")]
mod docs;
mod effects;
mod encode_;
pub mod gpu;
mod interop;
mod modules;
mod pathops;
mod prelude;
pub(crate) mod private;
pub mod skottie;
pub mod svg;
pub mod wrapper;
// TODO: We don't export utils/* into the crate's root yet. Should we?
pub mod utils;

#[macro_use]
extern crate bitflags;

// Prelude re-exports
pub use crate::prelude::{Borrows, ConditionallySend, Handle, RCHandle, RefHandle, Sendable};

// All Sk* types are accessible via skia_safe::
pub use crate::core::*;
#[cfg(feature = "pdf")]
pub use docs::*;
pub use effects::*;
pub use encode_::*;
#[allow(unused_imports)]
pub use modules::*;
pub use pathops::*;

#[cfg(test)]
mod transmutation_tests {
    use crate::{Point, prelude::NativeTransmutableSliceAccess};
    use skia_bindings::SkPoint;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_transmutation_of_fixed_size_arrays_to_slice() {
        let mut points = [Point::default(); 4];

        let points_native = points.native_mut();
        let native_point = SkPoint { fX: 10.0, fY: 11.0 };
        points_native[1] = native_point;

        assert_eq!(points[1].x, native_point.fX);
        assert_eq!(points[1].y, native_point.fY);
    }
}
