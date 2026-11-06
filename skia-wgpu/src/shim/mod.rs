//! The shim: the WebGPU C functions that Skia's Dawn backend calls, implemented on `wgpu`.
//!
//! This source is compiled once per supported `wgpu` major version. The parent module binds the
//! name `wgpu` to that version's crate, so nothing in here names a version. It also provides
//! `compat`, which makes up for where that version deviates from WebGPU (ADR 0003).
//!
//! Each shim function is exported under its C name from `webgpu.h`, such as
//! `wgpuDeviceCreateBuffer`, so that the linker resolves Skia's calls to it (ADR 0004). A constant
//! next to each function assigns it to its function pointer type from `webgpu.h`
//! (`WGPUProcDeviceCreateBuffer`), so the compiler checks its signature against the header.
//!
//! Every WebGPU object that Skia holds is an [`std::sync::Arc`] of a shim type, passed to Skia as
//! the opaque C handle (see `handle`). Skia's `AddRef` and `Release` calls change the strong count
//! of that `Arc`.

// The shim functions carry the C names of `webgpu.h`, which are not snake case.
#![allow(non_snake_case)]

use super::{compat, wgpu};

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
#[cfg(test)]
mod testing;
mod texture;
