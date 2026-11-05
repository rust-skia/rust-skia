//! Runs Skia's Graphite Dawn backend on the `wgpu` crate (ADR 0003).
//!
//! Skia's Dawn backend calls the WebGPU C functions of Dawn's `webgpu.h`. This crate exports them
//! (ADR 0004), and each of them calls `wgpu`. These functions are called the shim. The linker
//! resolves the calls of `skia-bindings` to them.
//!
//! Each supported `wgpu` major version is a module `vNN` behind the feature `wgpu-NN`. All of them
//! compile the same shim source in `src/shim`, in which the name `wgpu` refers to the version of
//! the module. A build enables exactly one of them, so the crate root re-exports its API, and
//! application code names the `wgpu` version only in its `Cargo.toml`.

#[cfg(feature = "wgpu-30")]
mod v30;
#[cfg(feature = "wgpu-30")]
pub use v30::*;
