//! Build support functions for the Rust-Skia library.

pub mod android;
pub mod binaries_config;
pub mod cargo;
pub mod clang;
pub mod features;
pub mod ios;
pub mod llvm;
pub mod skia;
pub mod skia_c_bindings;
pub mod vs;
pub mod xcode;

#[cfg(feature = "binary-cache")]
pub mod binary_cache;
