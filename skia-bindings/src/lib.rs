// 32-bit windows needs `thiscall` support.
// https://github.com/rust-skia/rust-skia/issues/540
#![cfg_attr(all(target_os = "windows", target_arch = "x86"), feature(abi_thiscall))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
pub use bindings::*;

mod defaults;
pub use defaults::*;

mod impls;
pub use impls::*;

#[cfg(feature = "textlayout")]
pub mod icu;

#[allow(unused_imports)]
#[doc(hidden)]
#[cfg(feature = "use-system-jpeg-turbo")]
use mozjpeg_sys;

#[cfg(test)]
#[path = "../build_support"]
mod build_support {
    pub mod binaries_config;
    #[cfg(feature = "binary-cache")]
    pub mod binary_cache;
    pub mod cargo;
    pub mod clang;
    pub mod features;
    pub mod platform;
    pub mod skia;
    pub mod skia_bindgen;
}
