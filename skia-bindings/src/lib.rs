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
#[allow(unused_imports)]
pub use defaults::*;

mod impls;

#[cfg(feature = "textlayout")]
pub mod icu;

#[allow(unused_imports)]
#[doc(hidden)]
#[cfg(feature = "use-system-jpeg-turbo")]
use mozjpeg_sys;
