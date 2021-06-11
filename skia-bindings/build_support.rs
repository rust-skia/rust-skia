//! Build support functions for the Rust-Skia library.

pub mod android;
pub mod cargo;
pub mod clang;
pub mod ios;
pub mod llvm;
pub mod skia;
pub mod vs;
pub mod xcode;

#[cfg(feature = "binary-cache")]
pub mod binary_cache;
