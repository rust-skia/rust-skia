//! The shim compiled against `wgpu` 30.

use wgpu30 as wgpu;

#[path = "shim/mod.rs"]
mod shim;

pub use shim::api::*;
