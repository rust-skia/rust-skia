// 32-bit windows needs `thiscall` support.
// https://github.com/rust-skia/rust-skia/issues/540
#![cfg_attr(all(target_os = "windows", target_arch = "x86"), feature(abi_thiscall))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
// https://github.com/rust-lang/rust-bindgen/issues/1651
#![allow(unknown_lints)]
#![allow(deref_nullptr)]
// GrVkBackendContext contains u128 fields on macOS
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/skia/bindings.rs"));

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
