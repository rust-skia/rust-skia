//! The shim: the WebGPU C functions that Skia's Dawn backend calls, implemented on `wgpu`.
//!
//! This source is compiled once per supported `wgpu` major version. The parent module binds the
//! name `wgpu` to that version's crate, so nothing in here names a version.
//!
//! Every WebGPU object that Skia holds is an [`std::sync::Arc`] of a shim type, passed to Skia as
//! the opaque C handle (see `handle`). Skia's `AddRef` and `Release` calls change the strong count
//! of that `Arc`.

use super::wgpu;

pub mod api;
mod binding;
mod buffer;
mod convert;
mod device;
mod encoder;
mod future;
mod handle;
mod instance;
mod pipeline;
mod queue;
mod table;
#[cfg(test)]
mod testing;
mod texture;
