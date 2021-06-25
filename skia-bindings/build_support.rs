//! Build support functions for the Rust-Skia library.

pub mod android;
pub mod binaries_config;
pub mod bind_skia;
pub mod build_skia;
pub mod cargo;
pub mod clang;
pub mod features;
pub mod ios;
pub mod xcode;

#[cfg(feature = "binary-cache")]
pub mod binary_cache;
