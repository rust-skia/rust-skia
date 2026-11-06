//! The shim compiled against `wgpu` 30.

/// The `wgpu` crate that this version's API takes, so that callers use the same version.
pub use wgpu30 as wgpu;

mod compat;

#[path = "shim/mod.rs"]
mod shim;

pub use shim::api::*;
