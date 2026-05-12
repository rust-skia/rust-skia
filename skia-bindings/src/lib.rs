#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
// <https://github.com/rust-lang/rust-bindgen/issues/1651>
#![allow(unknown_lints)]
#![allow(deref_nullptr)]
// GrVkBackendContext contains u128 fields on macOS
#![allow(improper_ctypes)]
#![allow(dead_code)]

// The following type aliases are needed because of name mangling changes introduced with clang 18,
// (this works together with `ITEM_RENAMES` in `skia_bindgen.rs`)
type std___1_string_view = std_string_view;
type std___2_string_view = std_string_view;
type std___1_string = std_string;
type std___2_string = std_string;

// Keep the target-specific shim crate linked so its exported WASI symbols resolve during final
// `wasm32-unknown-unknown` linking.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use skia_wasm_shims as _;
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "gl"))]
pub use skia_wasm_shims::{drop_gl_context, register_gl_context, set_gl_context, web_sys_get_proc};

include!(concat!(env!("OUT_DIR"), "/skia/bindings.rs"));

mod defaults;
#[allow(unused_imports)]
pub use defaults::*;

mod impls;
#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
mod wasm_alloc_shim;

#[cfg(feature = "textlayout")]
pub mod icu;

#[allow(unused_imports)]
#[doc(hidden)]
#[cfg(feature = "use-system-jpeg-turbo")]
use mozjpeg_sys;
